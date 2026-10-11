default:
    @just --list

run *args:
    cargo run -- {{args}}

build:
    cargo build --release

test:
    cargo test

check:
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings

install:
    cargo install --path .
