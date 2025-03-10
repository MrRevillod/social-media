# -----------------------------------------------------------------------
# BUILDER STAGE ---------------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.84-bullseye AS builder

WORKDIR /social-net

COPY Cargo.toml Cargo.lock ./

COPY apps/auth/Cargo.toml ./apps/auth/Cargo.toml
COPY apps/mailer/Cargo.toml ./apps/mailer/Cargo.toml
COPY libs/common/Cargo.toml ./libs/common/Cargo.toml
COPY libs/sqlx/Cargo.toml ./libs/sqlx/Cargo.toml

RUN mkdir -p ./apps/auth/src && echo 'fn main() {}' > ./apps/auth/src/main.rs
RUN mkdir -p ./apps/mailer/src && echo 'fn main() {}' > ./apps/mailer/src/main.rs
RUN mkdir -p ./libs/common/src && echo 'fn main() {}' > ./libs/common/src/lib.rs
RUN mkdir -p ./libs/sqlx/src && echo 'fn main() {}' > ./libs/sqlx/src/lib.rs

RUN cargo build --release -p mailer

RUN rm -rf ./apps
RUN rm -rf ./libs

COPY libs ./libs
COPY apps ./apps

# -----------------------------------------------------------------------
# DEVELOPMENT STAGE -----------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.84-bullseye AS dev

RUN cargo install cargo-watch

COPY --from=builder /social-net/ /social-net/

WORKDIR /social-net/apps/mailer

EXPOSE 8000

CMD ["cargo", "watch", "-x", "run"]

# -----------------------------------------------------------------------
# TEST STAGE ------------------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.84-bullseye AS test

COPY --from=builder /social-net/ /social-net/

WORKDIR /social-net/apps/mailer

EXPOSE 8000

CMD ["cargo", "test"]

# -----------------------------------------------------------------------
# PRODUCTION STAGE ------------------------------------------------------
# -----------------------------------------------------------------------

FROM debian:bullseye-slim AS prod

COPY --from=builder /social-net/target/release/mailer /usr/local/bin/mailer

EXPOSE 8000

CMD ["mailer"]

# ----------------------------------------------------------------------
