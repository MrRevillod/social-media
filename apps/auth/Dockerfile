
# BUILDER STAGE

FROM rust:1.84-bullseye as builder

RUN cargo install cargo-watch

WORKDIR /social-net

COPY Cargo.toml Cargo.lock ./

COPY apps/auth/Cargo.toml ./apps/auth/Cargo.toml
COPY libs/common/Cargo.toml ./libs/common/Cargo.toml

RUN mkdir -p ./apps/auth/src && echo 'fn main() {}' > ./apps/auth/src/main.rs
RUN mkdir -p ./libs/common/src && echo 'fn main() {}' > ./libs/common/src/lib.rs

RUN cargo build --release -p auth

RUN rm -rf ./apps
RUN rm -rf ./libs

COPY libs ./libs
COPY apps ./apps

# DEVELOPMENT STAGE

FROM rust:1.84-bullseye as dev

COPY --from=builder /social-net/ /social-net/

WORKDIR /social-net/apps/auth

EXPOSE 8000

CMD ["cargo", "watch", "-x", "run"]

# PRODUCTION STAGE

FROM debian:bullseye-slim as prod

COPY --from=builder /social-net/target/release/auth /usr/local/bin/auth

EXPOSE 8000

CMD ["auth"]
