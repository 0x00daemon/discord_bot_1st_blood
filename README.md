# CTFd First Blood Bot

A simple webhook based Discord bot to announce CTFd first bloods.

By default, the bot skips any first bloods achieved before it was run but can be configured to announce existing solves on CTFd.

## Command Options

```
Usage: ctfd-solve-announcer-discord [OPTIONS] --webhook-url <WEBHOOK_URL> --ctfd-url <CTFD_URL> --ctfd-api-key <CTFD_API_KEY>

Options:
  -w, --webhook-url <WEBHOOK_URL>
          Discord Webhook URL
  -c, --ctfd-url <CTFD_URL>
          CTFd URL
  -a, --ctfd-api-key <CTFD_API_KEY>
          CTFd API Key
      --announce-existing
          Announce existing solves from before bot was run
  -r, --refresh-interval-seconds <REFRESH_INTERVAL_SECONDS>
          Refresh interval in seconds [default: 5]
  -h, --help
          Print help
  -V, --version
          Print version
```

To create a Discord webhook URL, go to `Server Settings` -> `Integrations` -> `Webhooks` -> `New Webhook`.
Choose a name for it (will show up as the sender of each solve message) and set the channel for the messages.
Then copy the webhook URL.

To create a CTFd API key, make a profile on your CTFd, choose `Settings` -> `Access Tokens`, click `Generate` and copy the token.

## Local Usage

Install directly from GitHub with

```bash
cargo install --git https://github.com/Nissen96/ctfd-solve-announcer-discord
```

or clone repo and install from within `ctfd-solve-announcer-discord/` with

```bash
cargo install --path .
```

Run with

```bash
ctfd-solve-announcer-discord --help
```

## Docker Usage

The Dockerfile is configured to run the bot with default optional options.
To modify these, change `CMD` in the Dockerfile before building.

Build the docker image with

```bash
docker build --tag ctfdsolveannouncerdiscord .
```

Run a container using the created image and provide the required options as environment variables:

```bash
docker run -d --name ctfd-solve-announcer-discord \
    -e WEBHOOK_URL=<DISCORD_WEBHOOK_URL> \
    -e CTFD_URL=<CTFD_APP_URL> \
    -e CTFD_API_KEY=<CTFD_API_KEY> \
    ctfdsolveannouncerdiscord
```

- To create a Discord webhook URL, go to `Server Settings` -> `Integrations` -> `Webhooks` -> `New Webhook`.
- Choose a name for it (will show up as the sender of each solve message) and set the channel for the messages.
- Then copy the webhook URL.
- For the CTFD_URL, just put your ctfd website url.
- To create a CTFd API key, make a profile on your CTFd, choose `Settings` -> `Access Tokens`, click `Generate` and copy the token.

**Note:** When testing on a local CTFd instance running on `localhost`, make sure to replace `localhost` with `host.docker.internal` in `CTFD_URL`.
