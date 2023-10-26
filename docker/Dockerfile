FROM rust:1.73.0-slim-buster as builder
RUN apt update && apt install -y git

WORKDIR /build
COPY .. .
# Get the docker image architecture instead of the default host architecure from rustc
RUN echo "$(rustc -vV | sed -n 's|host: ||p')" > rust_target
RUN cargo build --target $(cat rust_target) --release --locked
RUN mv /build/target/$(cat rust_target)/release/cog /

FROM debian:buster-slim

RUN apt update && apt install -y git

# See https://git-scm.com/docs/git-config#Documentation/git-config.txt-safedirectory
RUN echo '[safe]\n\tdirectory = *' > /etc/gitconfig

COPY --from=builder /cog /usr/local/bin/cog
COPY docker/entrypoint.sh .

WORKDIR /app
ENTRYPOINT ["/entrypoint.sh"]
