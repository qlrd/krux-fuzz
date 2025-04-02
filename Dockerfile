FROM rustlang/rust:nightly

RUN apt-get update && apt-get install -y \
  python3 python3-pip python3-venv \
  git build-essential automake libtool pkg-config \
  && rm -rf /var/lib/apt/lists/*

RUN git clone https://github.com/bitcoin-core/secp256k1.git && \
  cd secp256k1 && ./autogen.sh && \
  ./configure --enable-module-recovery --enable-experimental \
  --enable-module-ecdh --enable-module-schnorrsig && \
  make && make install

WORKDIR /app
COPY . .

RUN mkdir /app/fuzz/logs && \
  rustup default nightly && \
  rustup component add rustfmt clippy && \
  cargo install cargo-fuzz just

RUN python3 -m venv .venv && \
  . .venv/bin/activate

RUN just clean && just prepare

CMD just list
