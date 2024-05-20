clean-targets:
    rm -rd target/release
    rm target/.rustc_info.json

test:
    cargo install cargo-nextest
    cargo nextest run

json-schema: 
    cargo run -p cocogitto-schema > docs/website/.vuepress/public/cog-schema.json

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

changelog-inferno:
    cargo build --release
    perf record --call-graph dwarf target/release/cog changelog
    perf script | inferno-collapse-perf > stacks.folded
    cat stacks.folded | inferno-flamegraph > flamegraph.svg
    chromium flamegraph.svg

changelog-massif:
    valgrind --tool=massif cog changelog

