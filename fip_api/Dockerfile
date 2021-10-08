# FROM docker.io/curlimages/curl:7.77.0 as linkerd
# ARG LINKERD_AWAIT_VERSION=v0.2.3
# ENV LINKERD_AWAIT_VERSION=${LINKERD_AWAIT_VERSION}
# RUN curl -sSLo /tmp/linkerd-await https://github.com/linkerd/linkerd-await/releases/download/release%2F${LINKERD_AWAIT_VERSION}/linkerd-await-${LINKERD_AWAIT_VERSION}-amd64 && \
#     chmod 755 /tmp/linkerd-await

FROM rust:1.55.0-buster as builder
WORKDIR /fip_api
ADD . .
RUN rustup component add rustfmt
RUN cargo build --release --package fip_api

FROM gcr.io/distroless/cc:nonroot
# COPY --from=linkerd /tmp/linkerd-await /linkerd-await
COPY --from=builder /fip/fip_api/auth_model.conf /
COPY --from=builder /fip/fip_api/auth_policy.csv /
COPY --from=builder /fip/target/release/fip_api /
USER 65532
# ENTRYPOINT ["/linkerd-await", "--shutdown", "--"]
CMD ["/fip_api"]

# # FROM docker.io/curlimages/curl:7.77.0 as linkerd
# # ARG LINKERD_AWAIT_VERSION=v0.2.3
# # ENV LINKERD_AWAIT_VERSION=${LINKERD_AWAIT_VERSION}
# # RUN curl -sSLo /tmp/linkerd-await https://github.com/linkerd/linkerd-await/releases/download/release%2F${LINKERD_AWAIT_VERSION}/linkerd-await-${LINKERD_AWAIT_VERSION}-amd64 && \
# #     chmod 755 /tmp/linkerd-await

# FROM rust:1.55.0-buster as builder
# RUN apt-get update && \
#     apt-get install -y \
#         binutils-aarch64-linux-gnu \
#         binutils-arm-linux-gnueabihf \
#         binutils-x86-64-linux-gnu \
#         docker.io \
#         jq \
#         musl-tools \
#         && \
#     apt-get clean && \
#     rm -fr \
#         /var/lib/apt/lists/* \
#         /tmp/* \
#         /var/tmp/
# # RUN cargo install --locked cross
# WORKDIR /fip_api
# ADD . .
# RUN rustup component add rustfmt
# RUN cargo build --package fip_api --release --target x86_64-unknown-linux-musl

# FROM scratch
# # COPY --from=linkerd /tmp/linkerd-await /linkerd-await
# COPY --from=builder /fip/fip_api/auth_model.conf /
# COPY --from=builder /fip/fip_api/auth_policy.csv /
# COPY --from=builder /fip_api/target/x86_64-unknown-linux-musl/release/fip_api /
# # USER 65532
# # ENTRYPOINT ["/linkerd-await", "--shutdown", "--"]
# CMD ["/fip_api"]
