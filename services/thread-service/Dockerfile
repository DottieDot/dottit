FROM lukemathwalker/cargo-chef:latest-rust-1.64.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 

ENV RUST_BACKTRACE=1

COPY ./rust-toolchain.toml . 

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json

# Build application
COPY . .

RUN cargo build --bin web

ENTRYPOINT [ "cargo", "run", "--bin", "web" ]
