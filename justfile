# List the available commands
default:
  @just --list

clean:
  cargo clean
  rm -rf crates/call_python_script/target/**
  rm -rf fuzz/target/**
  rm -rf diybitcoinhardware
  rm -rf selfcustody
  rm -rf BBQr
  rm -rf fuzz/artifacts
  rm -rf fuzz/corpus

lint:
  cargo +nightly fmt --all && cargo +nightly clippy --all-targets

format:
  cargo +nightly fmt --all --check

prepare:
  cargo +nightly run --bin prepare-fuzz

list:
  cargo +nightly fuzz list

diff_base32: 
  cargo +nightly fuzz run diff_base32

diff_bip39:
  cargo +nightly fuzz run diff_bip39

