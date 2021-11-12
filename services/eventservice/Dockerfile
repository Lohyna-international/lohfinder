FROM rust:latest

WORKDIR /usr/src/app

RUN rustup default nightly
COPY ./src ./src
COPY ./Cargo.toml .
COPY ./start.sh ./start.sh
RUN cargo build --release
RUN ls
ENTRYPOINT [ "/bin/bash", "start.sh" ]


