FROM rust:1.84-bullseye

RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /social-net

COPY . .

CMD ["sqlx"]