FROM rust:1.63-slim

RUN apt-get update && apt-get install -y \
    libgpiod-dev \
    --no-install-recommends && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/gecko-client

COPY . .

RUN cargo build --release --target-dir /usr/src/gecko-client/output/

CMD [ "/usr/src/gecko-client/output/release/gecko-client" ]
