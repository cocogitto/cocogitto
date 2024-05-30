# Note that the following build needs binaries to be precompiled for the target
# architectures. Use the `build-all` just recipes to build for all targets.
FROM alpine as arm-builder
COPY ./target/armv7-unknown-linux-musleabihf/release/cog /cog

FROM alpine as amd64-builder
COPY ./target/x86_64-unknown-linux-musl/release/cog /cog

FROM alpine as arm64-builder
COPY ./target/aarch64-unknown-linux-gnu/release/cog /cog

FROM ${TARGETARCH}-builder AS builder

FROM alpine
RUN  apk add --no-cache git

COPY --from=builder /cog /usr/bin/cog

# See https://git-scm.com/docs/git-config#Documentation/git-config.txt-safedirectory
COPY docker/gitconfig /etc/gitconfig

WORKDIR /app
ENTRYPOINT ["cog"]
