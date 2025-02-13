
FROM rust:1.84-bullseye

WORKDIR /social-net

COPY . .

CMD ["cargo", "prisma-migrate"]
ENTRYPOINT ["cargo", "prisma-generate"]
