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
WORKDIR /app/services/mediator
COPY ./services/mediator .

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
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo chef cook --recipe-path recipe.json

# Copy shared crates
COPY ./shared/shared-rust .

# Build shared crates
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build

############
# Service  #
############
WORKDIR /app/services/mediator

# Copy toolchain
COPY ./services/mediator/rust-toolchain.toml . 

# Get cargo chec recipe
COPY --from=planner /app/services/mediator/recipe.json recipe.json

# Build and Cache dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo chef cook --recipe-path recipe.json

# Copy service
COPY ./services/mediator .

# Build service
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --bin web

# Run service
ENTRYPOINT [ "cargo", "run", "--bin", "web" ]
