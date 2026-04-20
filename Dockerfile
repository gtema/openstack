################
##### chef
FROM rust:1.95.0-slim-trixie AS base

RUN cargo install --locked cargo-chef

WORKDIR /app

################
##### Planner
FROM base AS planner

COPY . .

# Prepare the build recipe
RUN cargo chef prepare --recipe-path recipe.json

################
##### Builder
FROM base AS builder

RUN rustup target add x86_64-unknown-linux-musl &&\
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

# Copy the build recipe
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies with the cargo-chef
RUN cargo chef cook --target x86_64-unknown-linux-musl --release --recipe-path recipe.json

# # Now copy in the rest of the sources
COPY . .
# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release --bin osc

################
##### Runtime
FROM alpine:3.23.0@sha256:51183f2cfa6320055da30872f211093f9ff1d3cf06f39a0bdb212314c5dc7375 AS runtime

LABEL maintainer="Artem Goncharov"

RUN apk add --no-cache bash

# Copy application binary from builder image
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/osc /usr/local/bin
