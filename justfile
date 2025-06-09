# prettier-ignore
default:
  just dev

build:
  cargo build

test:
  cargo test

clippy:
  cargo clippy --all-targets -- -D warnings

fmt:
  cargo fmt --all

check:
  cargo check

dev:
  cargo check --all-targets
  cargo test --workspace
  cargo clippy --all-targets -- -D warnings
