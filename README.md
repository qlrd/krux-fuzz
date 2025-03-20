# Krux fuzz

This project is inspired on [bitcoinfuzz](https://github.com/brunoerg/bitcoinfuzz).

It's aimed to get, **from source in main branch**, pieces of codes from
[Krux](https://github.com/selfcustody/krux) that follow some standards -- like BBQr, BIP39,
encryption, etc. --  and do [differential fuzzing](https://en.wikipedia.org/wiki/Differential_testing),
i.e., comparing the krux's output with some other outputs, mainly, rust libraries.

If we get some error, maybe we found a bug :)

## Prepare environment

To use `krux-fuzz` you'll need both python and rust.

### Rust

```bash
# install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# setup rust with the latest rust toolchain
rustup install nightly

# check if it's the default toolchain
rustup default nightly

# add some task manager, linter and formatter
rustup component add clippy
rustup component add rustfmt

# install task manager tool
cargo install just

# install fuzz tool
cargo install cargo-fuzz
```

### Python

To not break your system, we recomment to use `virtualenv`:

```bash
# install virtualenv
python -m pip install virtualenv

# create a new virtual environment
python -m venv <path-of-environment>

# load virtual environment
source <path-of-environment>/bin/activate

# check the new environment
which python
```

## Download source

```bash
# this will change to selfcustody/krux-fuzz in future
git clone https://github.com/qlrd/krux-fuzz.git
```

## Download krux sources

This will download some krux sources:

```bash
just prepare
```

## Tests

This will list some fuzz tests:

```
just fuzz
```

## DONE

- [`krux/src/bbqr.py`](https://github.com/selfcustody/krux/blob/main/src/krux/bbqr.py): [BBQR](https://bbqr.org/) encoding/decoding with base32 uppercase format;

## BAKING...

- None :/

## TODO

- [`krux/src/bip39.py`](https://github.com/selfcustody/krux/blob/main/src/krux/bip39.py): [BIP39](https://en.bitcoin.it/wiki/BIP_0039) Mnemonic code for generating deterministic keys;

- [`krux/src/encryption.py`](https://github.com/selfcustody/krux/blob/main/src/krux/encryption.py): Encryption standards with AES-ECB, AES-CBC;

- [`krux/src/psbt.py`](https://github.com/selfcustody/krux/blob/main/src/krux/psbt.py): [Partially Signed  Bitcoin Transaction](https://github.com/bitcoin/bitcoin/blob/master/doc/psbt.md);

- Add one new here...
