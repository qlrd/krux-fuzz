# List the available commands
default:
  @just --list

clean:
    cargo clean

lint:
    cargo +nightly fmt --all && cargo +nightly clippy --all-targets

format:
    cargo +nightly fmt --all --check

prepare:
    cargo run --bin prepare-fuzz

fuzz: 
    cargo fuzz run diff_base32
