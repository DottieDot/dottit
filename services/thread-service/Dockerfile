FROM rust:1.64-buster as builder

WORKDIR /app

RUN apt-get update
RUN apt-get install -y postgresql

COPY . .

RUN cargo build

ENV RUST_BACKTRACE=1
ENTRYPOINT [ "cargo", "run", "--bin", "web" ]
