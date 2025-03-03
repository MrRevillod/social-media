
# -----------------------------------------------------------------------
# COMMON STAGE ----------------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.84-bullseye

RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /social-net

COPY . .
COPY libs/sqlx/init-db.sh /social-net/libs/sqlx/init-db.sh

RUN chmod +x /social-net/libs/sqlx/init-db.sh

# Add sqlx command aliases

RUN echo "alias sqlx-create='sqlx database create'" >> ~/.bashrc && \
    echo "alias sqlx-migrate='sqlx migrate run --source /social-net/libs/sqlx/migrations'" >> ~/.bashrc && \
    echo "source ~/.bashrc" >> ~/.bashrc

# With this we can run the sqlx commands from the container like this:

# docker compose run sqlx_cli_[env] sqlx-create
# docker compose run sqlx_cli_[env] sqlx-migrate

# Run init-db.sh and then start a bash shell

CMD ["/bin/bash", "-c", "/social-net/libs/sqlx/init-db.sh && /bin/bash"]
