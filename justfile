default:
  @just --list --unsorted

# Format code with rust
fmt:
  cargo fmt

# Lint code with clippy
lint:
  cargo fmt --all -- --check
  cargo clippy --all-targets --all-features

# Run unit tests against the current platform
unit-test:
  cargo nextest run --no-capture

run year day part input:
  cargo run -- --year {{ year }} --day {{ day }} --part {{ part }} --input {{ input }}

