
FROM rust:1.84-bullseye

WORKDIR /diesel_cli

COPY ./libs/database .

RUN cargo install diesel_cli --no-default-features --features postgres

