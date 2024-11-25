mod ctfd;

use std::collections::HashMap;

use ctfd::{CTFdClient, ChallengeSolver};
use serenity::http::Http;
use serenity::model::webhook::Webhook;

use clap::Parser;
use rusqlite::Connection;

/// A Discord webhook bot to announce CTFd first bloods
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Discord Webhook URL
    #[arg(short, long)]
    webhook_url: String,

    /// CTFd URL
    #[arg(long, short = 'c')]
    ctfd_url: String,

    /// CTFd API Key
    #[arg(long, short = 'a')]
    ctfd_api_key: String,

    /// Announce existing first bloods from before bot was run
    #[arg(long, action)]
    announce_existing: bool,

    /// Refresh interval in seconds
    #[arg(short, long, default_value = "5")]
    refresh_interval_seconds: u64,
}

async fn populate_announced_solves(
    ctfd_client: &CTFdClient,
    announced_solves: &mut HashMap<i64, Vec<ChallengeSolver>>,
) {
    // Get a list of all challenges
    let challenges = ctfd_client.get_challenges().await.unwrap();

    for challenge in challenges {
        // Get any solver of the challenge
        let potential_solver = challenge.get_solves(ctfd_client).await.unwrap().into_iter().nth(0);
        if potential_solver == None {
            continue;
        }

        // Add the solve to the list of announced solves
        let solver = potential_solver.unwrap();
        announced_solves
            .entry(challenge.id)
            .or_insert_with(Vec::new)
            .push(solver);
    }
}

async fn announce_solves(
    http: &Http,
    webhook: &Webhook,
    ctfd_client: &CTFdClient,
    announced_solves: &mut HashMap<i64, Vec<ChallengeSolver>>,
    db_conn: &Connection,
) {
    // Get a list of all challenges
    let challenges = ctfd_client.get_challenges().await.unwrap();

    for challenge in challenges {
        // Skip already announced first bloods
        if announced_solves.contains_key(&challenge.id) {
            continue;
        }

        // Get first solver of challenge
        let potential_solver = challenge.get_solves(ctfd_client).await.unwrap().into_iter().nth(0);
        
        // Skip unsolved
        if potential_solver == None {
            continue;
        }
        let solver = potential_solver.unwrap();

        // Check if the solve is new
        if !announced_solves
            .get(&challenge.id)
            .unwrap_or(&Vec::new())
            .contains(&solver)
        {
            println!("Announcing first blood for {} by {}", challenge.name, solver.name);

            // Send a message to the webhook
            webhook
                .execute(&http, false, |w| {
                    // If this is the first solve
                    if !announced_solves.contains_key(&challenge.id) {
                        w.content(format!(
                            ":drop_of_blood: First blood for **{}** goes to **{}**! :drop_of_blood:",
                            challenge.name, solver.name
                        ))
                    } else {
                        w.content(format!("{} just solved {}! :tada:", solver.name, challenge.name))
                    }
                })
                .await
                .expect("Could not execute webhook.");

            // Add the solve to the database
            db_conn
                .execute(
                    "INSERT INTO announced_solves (challenge_id, solver_id) VALUES (?1, ?2);",
                    (&challenge.id, &solver.account_id),
                )
                .unwrap();

            // Add the solve to the list of announced solves
            announced_solves
                .entry(challenge.id)
                .or_insert_with(Vec::new)
                .push(solver);
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Starting CTFd Discord First Blood Bot...");

    let args = Args::parse();

    let http = Http::new("");
    let webhook = Webhook::from_url(&http, &args.webhook_url)
        .await
        .expect("Supply a webhook url");

    let ctfd_client = CTFdClient::new(args.ctfd_url, args.ctfd_api_key);

    // A hashmap of challenge id to their solvers
    let mut announced_solves: HashMap<i64, Vec<ChallengeSolver>> = HashMap::new();

    println!("Connecting to sqlite3 db...");
    let db_conn = Connection::open("ctfd_discord.sqlite3").unwrap();

    println!("Creating table of announced solves in db...");
    db_conn
    .execute("CREATE TABLE IF NOT EXISTS announced_solves (id INTEGER PRIMARY KEY AUTOINCREMENT, challenge_id INTEGER, solver_id INTEGER);", ())
    .unwrap();

    // Populate the announced solves hashmap with the existing solves
    let mut statement = db_conn
        .prepare("SELECT challenge_id, solver_id FROM announced_solves;")
        .unwrap();

    let announced_iter = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0).unwrap(),
                ChallengeSolver {
                    account_id: row.get::<_, i64>(1).unwrap(),
                    name: "".to_string(),
                },
            ))
        })
        .unwrap();

    for announced in announced_iter {
        let (challenge_id, solver) = announced.unwrap();

        announced_solves
            .entry(challenge_id)
            .or_insert_with(Vec::new)
            .push(solver);
    }

    // Skips announcing existing solves by default
    if !args.announce_existing {
        println!("Announcing existing first bloods...");
        populate_announced_solves(&ctfd_client, &mut announced_solves).await;
    }

    println!("Bot running, waiting for first bloods...");

    loop {
        announce_solves(&http, &webhook, &ctfd_client, &mut announced_solves, &db_conn).await;
        tokio::time::sleep(std::time::Duration::from_secs(args.refresh_interval_seconds)).await;
    }
}
