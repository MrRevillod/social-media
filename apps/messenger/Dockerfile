FROM rust:1.84-bullseye

RUN cargo install cargo-watch

WORKDIR /chat

COPY Cargo.toml Cargo.lock ./

COPY apps ./apps
COPY libs ./libs

WORKDIR /chat/apps/messenger

CMD ["cargo", "watch", "-x", "run"]