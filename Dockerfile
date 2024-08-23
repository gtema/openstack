################
##### Builder
FROM rust:1.80.1-slim@sha256:e8e40c50bfb54c0a76218f480cc69783b908430de87b59619c1dca847fdbd753 as builder

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
COPY openstack_tui/build.rs /usr/src/openstack/openstack_tui/
COPY structable_derive/Cargo.toml /usr/src/openstack/structable_derive/
COPY xtask/Cargo.toml /usr/src/openstack/xtask/
COPY fuzz/Cargo.toml /usr/src/openstack/fuzz/
RUN mkdir -p openstack/openstack_cli/src/bin && touch openstack/openstack_cli/src/lib.rs &&\
    cp openstack/src/main.rs openstack/openstack_cli/src/bin/osc.rs &&\
    mkdir -p openstack/openstack_sdk/src && touch openstack/openstack_sdk/src/lib.rs &&\
    mkdir -p openstack/structable_derive/src && touch openstack/structable_derive/src/lib.rs &&\
    mkdir -p /usr/src/openstack/xtask/src && touch openstack/xtask/src/lib.rs &&\
    mkdir -p openstack/fuzz/src && touch openstack/fuzz/src/lib.rs

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
RUN touch openstack_sdk/src/lib.rs && touch openstack_cli/src/bin/osc.rs && touch openstack_cli/src/lib.rs && touch structable_derive/src/lib.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release -p openstack_cli

################
##### Runtime
FROM alpine:3.20.2@sha256:0a4eaa0eecf5f8c050e5bba433f58c052be7587ee8af3e8b3910ef9ab5fbe9f5 AS runtime

LABEL maintainer="Artem Goncharov"

RUN apk add --no-cache bash

# Copy application binary from builder image
COPY --from=builder /usr/src/openstack/target/x86_64-unknown-linux-musl/release/osc /usr/local/bin
