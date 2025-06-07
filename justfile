default: just dev

build: cargo build

test: cargo test

clippy: cargo clippy --all-targets -- -D warnings

fmt: cargo fmt --all

check: cargo check

dev: just build
  just test
  just clippy
