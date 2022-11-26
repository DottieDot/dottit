# THIS DOCKER FILE'S CONTEXT IS THE PROJECT ROOT

FROM rust:1.65

WORKDIR /app

# Copy general relevant files
COPY ./rust-toolchain.toml .
COPY ./rustfmt.toml .
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./shared/shared-rust ./shared/shared-rust

RUN rustup component add rustfmt

# Copy support files
COPY ./docker/support/services ./services

# Copy service files
COPY ./services/thread-service ./services/thread-service


# Build service
RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cargo build \
  --bin thread-service-web

ENTRYPOINT [ "cargo", "run", "--bin", "thread-service-web" ]
