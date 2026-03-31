################
##### chef
FROM rust:1.94.0-slim@sha256:d6782f2b326a10eaf593eb90cafc34a03a287b4a25fe4d0c693c90304b06f6d7 AS chef

RUN rustup target add x86_64-unknown-linux-musl &&\
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

RUN cargo install --locked cargo-chef
WORKDIR app

################
##### Planner
FROM chef as planner
COPY . .
# Prepare the build recipe
RUN cargo chef prepare --recipe-path recipe.json

################
##### Builder
FROM chef AS builder

RUN rustup target add x86_64-unknown-linux-musl &&\
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

## ## Install target platform (Cross-Compilation) --> Needed for Alpine
#RUN rustup target add x86_64-unknown-linux-musl

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
