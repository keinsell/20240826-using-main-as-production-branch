VERSION --global-cache 0.8
PROJECT keinsell/neuronek-cli
IMPORT github.com/earthly/lib/rust:3.0.1 AS rust

install:
  FROM rustlang/rust:nightly
  DO rust+INIT --keep_fingerprints=true
  DO rust+SET_CACHE_MOUNTS_ENV
  DO rust+CARGO --args="install cargo-binstall"
  DO rust+CARGO --args="binstall just -y"

source:
    FROM +install
    WORKDIR /tmp
    COPY --keep-ts Cargo.lock Cargo.toml
    COPY --keep-ts --dir src .

lint:
   FROM +source
   RUN rustup component add clippy
   DO rust+CARGO --args="clippy --all-features --all-targets -- -D warnings"

fmt:
  FROM +lint
  RUN rustup component add rustfmt
  DO rust+CARGO --args="fmt --check"

test:
    FROM +source
    DO rust+CARGO --args="test"

build:
    FROM +source
    COPY --keep-ts --dir src packages Cargo.lock Cargo.toml .
    DO rust+CARGO --args="build --release --bin neuronek" --output="release/[^/\.]+"
    DO rust+CROSS --args=""
    SAVE ARTIFACT target/release/neuronek neuronek

build-darwin-x86:
    FROM multiarch/crossbuild
    # FROM ghcr.io/darwin-containers/darwin-jail/ventura-arm64:latest
    ENV CROSS_TRIPPLE x86_64-apple-darwin
    RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain="nightly" -t "x86_64-apple-darwin"
    DO rust+INIT --keep_fingerprints=true
    WORKDIR /tmp
    COPY --keep-ts --dir src packages Cargo.lock Cargo.toml .
    DO rust+CARGO --args="build --release --target x86_64-apple-darwin"

all:
    # BUILD +lint
    # BUILD +fmt
    BUILD +build
    BUILD +test

docker:
    FROM registry.suse.com/bci/bci-micro:15.5
    COPY +build/neuronek neuronek
    ENTRYPOINT ["./neuronek"]
    SAVE IMAGE --push neuronek/cli:dev
