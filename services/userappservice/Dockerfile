FROM rust:latest

WORKDIR /usr/src/app

RUN rustup default nightly
COPY ./services/userappservice/src ./src
COPY ./services/userappservice/Cargo.toml .
COPY ./secrets/key.json ./key.json
RUN mkdir -p /data/uappservice/db
RUN cargo build --release

ENTRYPOINT [ "cargo", "run", "--release"]
