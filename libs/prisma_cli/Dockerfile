
FROM rust:1.84-bullseye

WORKDIR /social-net

COPY . .

CMD ["sh", "-c", "cargo prisma-migrate ; cargo prisma-generate"]
ENTRYPOINT ["./libs/prisma_cli/path_fix.sh"]
