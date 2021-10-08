ARG RUST_VERSION=1.54.0-buster
FROM rust:${RUST_VERSION}
RUN apt-get update && \
    apt-get install -y \
        binutils-aarch64-linux-gnu \
        binutils-arm-linux-gnueabihf \
        binutils-x86-64-linux-gnu \
        docker.io \
        jq \
        musl-tools \
        && \
    apt-get clean && \
    rm -fr \
        /var/lib/apt/lists/* \
        /tmp/* \
        /var/tmp/
ENV CROSS_DOCKER_IN_DOCKER=true
RUN cargo install --locked cross

# FROM buildpack-deps:stretch-curl

# ENV RUSTUP_HOME=/usr/local/rustup \
#     CARGO_HOME=/usr/local/cargo \
#     PATH=/usr/local/cargo/bin:$PATH

# RUN set -eux; \
#     url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
#     wget "$url"; \
#     chmod +x rustup-init; \
#     ./rustup-init -y --no-modify-path --default-toolchain nightly; \
#     rm rustup-init; \
#     chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
#     rustup --version; \
#     cargo --version; \
#     rustc --version;
