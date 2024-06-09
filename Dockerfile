FROM rust

WORKDIR /usr/src/gecko-client

COPY . .

RUN cargo build --release --target-dir /usr/src/gecko-client/output/

CMD [ "/usr/src/gecko-client/output/release/gecko-client" ]