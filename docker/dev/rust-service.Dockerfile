# THIS DOCKER FILE'S CONTEXT IS THE PROJECT ROOT

##############################
# BUILDER                    #
##############################
FROM rust:1.66 as builder

ARG SERVICE
ARG BINARY

ENV SERVICE ${SERVICE}
ENV BINARY ${BINARY}

WORKDIR /app

# Copy toolchain aind install rustfmt
COPY ./rust-toolchain.toml .
RUN rustup component add rustfmt

# Copy general relevant files
COPY ./rustfmt.toml .
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./shared/shared-rust ./shared/shared-rust


# Copy support files
COPY ./docker/support/services ./services

# Copy service files
COPY ./services/${SERVICE} ./services/${SERVICE}

RUN echo $BINARY && echo $SERVICE

# Build service
RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cargo build --bin $BINARY


# Copy binary to output directory
RUN --mount=type=cache,target=/app/target mkdir /output && cp /app/target/debug/$BINARY /output/$BINARY

##############################
# RUNTIME                    #
##############################
FROM debian as RUNTIME

WORKDIR /app

ARG BINARY
ENV BINARY ${BINARY}

ENV RUST_BACKTRACE 1

# Copy binary from builder
COPY --from=builder /output/${BINARY} ./entrypoint

# Expose web port
EXPOSE 8080

# Run app
CMD ["/app/entrypoint"]
