ARG RUST_VERSION=1.56.0-bullseye
FROM rust:${RUST_VERSION}
RUN apt-get update \
    && apt-get install -y binutils-aarch64-linux-gnu binutils-arm-linux-gnueabihf binutils-x86-64-linux-gnu cmake docker.io jq musl-tools \
    && apt-get clean \
    && rm -fr /var/lib/apt/lists/* /tmp/* /var/tmp/*
ENV CROSS_DOCKER_IN_DOCKER=true
RUN cargo install --locked cross
