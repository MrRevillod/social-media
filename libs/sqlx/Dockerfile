
# -----------------------------------------------------------------------
# COMMON STAGE ----------------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.84-bullseye AS common

RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /social-net

COPY . .
COPY libs/sqlx/init-db.sh /social-net/libs/sqlx/init-db.sh

RUN chmod +x /social-net/libs/sqlx/init-db.sh

# Add sqlx command aliases

RUN echo "alias sqlx-create='sqlx database create'" >> ~/.bashrc && \
    echo "alias sqlx-migrate='sqlx migrate run --source /social-net/libs/sqlx/migrations'" >> ~/.bashrc && \
    echo "source ~/.bashrc" >> ~/.bashrc

# -----------------------------------------------------------------------
# SEED STAGE ------------------------------------------------------------
# -----------------------------------------------------------------------

FROM common AS seed

RUN cargo install cargo-watch

ARG TARGET

ENV TARGET=${TARGET}

WORKDIR /social-net/libs/sqlx

ENTRYPOINT ["sh", "-c", "/social-net/libs/sqlx/init-db.sh && cargo watch -x 'run -- $TARGET'"]
