# ctfd-solve-announcer-discord

A simple webhook based Discord bot to announce CTFd solves

## Usage

Install with

```bash
cargo install --git https://github.com/Nissen96/ctfd-solve-announcer-discord
```

Run with

```bash
ctfd-solve-announcer-discord --help
```

## Dockerfile Usage

Build the docker image

```bash
docker build --tag ctfdsolveannouncerdiscord .
```

Run a container using the created image and provide the environment variables

```bash
docker run -d --name CtfdSolveAnnouncerDiscord \ 
    -e WEBHOOK_URL=<DISCORD_WEBHOOK_URL> \
    -e CTFD_URL=<YOUR_CTFD_APP_URL> \
    -e CTFD_API_KEY=<YOUR_CTFD_API_KEY> \
    ctfdsolveannouncerdiscord
```

To create a Discord webhook URL, go to Server Settings -> Integrations -> Webhooks -> New Webhook.
Choose a name for it (will show up as the sender of each solve message) and set the channel for the messages. Then copy the webhook URL.

To create a CTFd API key, make a profile on your CTFd, choose Settings -> Access Tokens, click Generate and copy the token.

Enjoy!

## Contributions

Welcome! :D
