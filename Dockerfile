################
##### Builder
FROM rust:1.91.0-slim@sha256:af95fd1bb203d15e0e82a3c2ade1799767aa99dd91a652ce044533d6582d7415 AS builder

RUN rustup target add x86_64-unknown-linux-musl &&\
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new openstack

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/openstack/
COPY openstack_sdk/Cargo.toml /usr/src/openstack/openstack_sdk/
COPY openstack_cli/Cargo.toml /usr/src/openstack/openstack_cli/
COPY openstack_tui/Cargo.toml /usr/src/openstack/openstack_tui/
COPY openstack_types/Cargo.toml /usr/src/openstack/openstack_types/
COPY xtask/Cargo.toml /usr/src/openstack/xtask/
COPY fuzz/Cargo.toml /usr/src/openstack/fuzz/
RUN mkdir -p openstack/openstack_cli/src/bin && touch openstack/openstack_cli/src/lib.rs &&\
    cp openstack/src/main.rs openstack/openstack_cli/src/bin/osc.rs &&\
    mkdir -p openstack/openstack_tui/src/bin && touch openstack/openstack_tui/src/lib.rs &&\
    cp openstack/src/main.rs openstack/openstack_tui/src/bin/ostui.rs &&\
    mkdir -p openstack/openstack_sdk/src && touch openstack/openstack_sdk/src/lib.rs &&\
    mkdir -p /usr/src/openstack/xtask/src && touch openstack/xtask/src/lib.rs &&\
    mkdir -p openstack/fuzz/src && touch openstack/fuzz/src/lib.rs &&\
    mkdir -p openstack/openstack_sdk/examples &&\
    mkdir -p openstack/openstack_types/src && touch openstack/openstack_types/src/lib.rs &&\
    touch openstack/openstack_sdk/examples/query_find.rs &&\
    touch openstack/openstack_sdk/examples/paged.rs &&\
    touch openstack/openstack_sdk/examples/query.rs &&\
    touch openstack/openstack_sdk/examples/ignore.rs

# Set the working directory
WORKDIR /usr/src/openstack
RUN rm -rf src

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

## This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release -p openstack_cli

# Now copy in the rest of the sources
COPY . /usr/src/openstack/

## Touch main.rs to prevent cached release build
RUN touch openstack_sdk/src/lib.rs && touch openstack_cli/src/bin/osc.rs && touch openstack_cli/src/lib.rs && touch openstack_types/src/lib.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release --bin osc

################
##### Runtime
FROM alpine:3.22.0@sha256:8a1f59ffb675680d47db6337b49d22281a139e9d709335b492be023728e11715 AS runtime

LABEL maintainer="Artem Goncharov"

RUN apk add --no-cache bash

# Copy application binary from builder image
COPY --from=builder /usr/src/openstack/target/x86_64-unknown-linux-musl/release/osc /usr/local/bin
