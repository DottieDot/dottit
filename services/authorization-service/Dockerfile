FROM lukemathwalker/cargo-chef:latest-rust-1.64.0 AS chef
WORKDIR /app

#####################################
# PLAN                              #
#####################################
FROM chef AS planner

############
# Shared   #
############
WORKDIR /app/shared/shared-rust
COPY ./shared/shared-rust .

RUN cargo chef prepare --recipe-path recipe.json

############
# Service  #
############
WORKDIR /app/services/authorization-service
COPY ./services/authorization-service .

RUN cargo chef prepare --recipe-path recipe.json


#####################################
# BUILD                             #
#####################################
FROM chef AS builder 

ENV RUST_BACKTRACE=1

############
# Shared   #
############
WORKDIR /app/shared/shared-rust

# Copy toolchain
COPY ./shared/shared-rust/rust-toolchain.toml . 

# Get cargo chef recipe
COPY --from=planner /app/shared/shared-rust/recipe.json recipe.json

# Delete cargo cache
RUN rm -rf ~/.cargo/registry/*

# Build and Cache dependencies
RUN cargo chef cook --recipe-path recipe.json

# Copy shared crates
COPY ./shared/shared-rust .

# Build shared crates
RUN cargo build

############
# Service  #
############
WORKDIR /app/services/authorization-service

# Copy toolchain
COPY ./services/authorization-service/rust-toolchain.toml . 

# Get cargo chec recipe
COPY --from=planner /app/services/authorization-service/recipe.json recipe.json

# Build and Cache dependencies
RUN cargo chef cook --recipe-path recipe.json

# Copy service
COPY ./services/authorization-service .

# Build service
RUN cargo build --bin web

# Run service
ENTRYPOINT [ "cargo", "run", "--bin", "web" ]
