clean-targets:
    rm -rd target/release
    rm target/.rustc_info.json

test:
    cargo install cargo-nextest
    cargo nextest run

build-x86:
    cross build --target x86_64-unknown-linux-musl --release
    just clean-targets

build-arm-v7:
    cross build --target armv7-unknown-linux-musleabihf --release
    just clean-targets

build-aarch64:
    cross build --target aarch64-unknown-linux-gnu --release
    just clean-targets

build-all: build-x86 build-arm-v7 build-aarch64

docker-build: build-all
    docker buildx build --platform linux/amd64,linux/arm/v7,linux/arm64/v8  . -t cocogitto/cog:latest -f docker/Dockerfile

