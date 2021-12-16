FROM rust:latest

WORKDIR /usr/src/app

RUN rustup default nightly
COPY ./services/userappservice/src ./src
COPY ./services/userappservice/Cargo.toml .
COPY ./services/userappservice/start.sh ./start.sh
COPY ./secrets/key.json ./key.json
RUN mkdir -p /data/uappservice/db
RUN cargo build --release

ENTRYPOINT [ "/bin/bash", "start.sh" ]
