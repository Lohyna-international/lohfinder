FROM rust:latest

WORKDIR /usr/src/app

RUN rustup default nightly
COPY ./services/eventservice/src ./src
COPY ./services/eventservice/Cargo.toml .
COPY ./secrets/key.json ./key.json
COPY ./services/eventservice/start.sh ./start.sh
RUN mkdir -p /data/eventservice/db
RUN cargo build --release

ENTRYPOINT [ "/bin/bash", "start.sh" ]
