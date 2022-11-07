FROM lukemathwalker/cargo-chef:latest-rust-1.64.0 AS chef
WORKDIR /app

#####################################
# PLAN                              #
#####################################
FROM chef AS planner

COPY . .

RUN rustup component add rustfmt

RUN cargo chef prepare --recipe-path recipe.json

#####################################
# BUILD                             #
#####################################
FROM chef AS builder

RUN curl -sSL https://rover.apollo.dev/nix/latest | sh

COPY rust-toolchain.toml .

RUN rustup component add rustfmt

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --recipe-path recipe.json

COPY . .

RUN cargo build --bin gateway-service

ENTRYPOINT [ "cargo", "run", "--bin", "gateway-service" ]
