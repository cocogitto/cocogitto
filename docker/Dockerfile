FROM rust:1.73.0-slim-buster as builder
RUN apt update && apt install -y git

WORKDIR /usr/src/cocogitto
COPY .. .
RUN cargo install --path .

FROM debian:buster-slim

RUN apt update && apt install -y git

# See https://git-scm.com/docs/git-config#Documentation/git-config.txt-safedirectory
RUN echo '[safe]\n\tdirectory = *' > /etc/gitconfig

COPY docker/entrypoint.sh .
COPY --from=builder /usr/local/cargo/bin/cog /usr/local/bin/cog
WORKDIR /app

ENTRYPOINT ["/entrypoint.sh"]
