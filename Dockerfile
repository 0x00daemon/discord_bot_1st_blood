FROM rust

RUN cargo install --git https://github.com/Nissen96/ctfd-solve-announcer-discord

CMD /usr/local/cargo/bin/ctfd-solve-announcer-discord --webhook-url $WEBHOOK_URL --ctfd-url $CTFD_URL --ctfd-api-key $CTFD_API_KEY --first-blood-only --dont-skip-existing-solves
