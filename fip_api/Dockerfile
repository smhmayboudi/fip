# FROM docker.io/curlimages/curl:7.77.0 as linkerd
# ARG LINKERD_AWAIT_VERSION=v0.2.3
# ENV LINKERD_AWAIT_VERSION=${LINKERD_AWAIT_VERSION}
# RUN curl -sSLo /tmp/linkerd-await https://github.com/linkerd/linkerd-await/releases/download/release%2F${LINKERD_AWAIT_VERSION}/linkerd-await-${LINKERD_AWAIT_VERSION}-amd64 && \
#     chmod 755 /tmp/linkerd-await

FROM rust:1.56.0-bullseye as builder
WORKDIR /fip_api
ADD . .
RUN rustup component add rustfmt
RUN cargo build --release --package fip_api

FROM gcr.io/distroless/cc-debian11:nonroot
# COPY --from=linkerd /tmp/linkerd-await /linkerd-await
COPY --from=builder /fip/fip_api/auth_model.conf /
COPY --from=builder /fip/fip_api/auth_policy.csv /
COPY --from=builder /fip/target/release/fip_api /
USER 65532
# ENTRYPOINT ["/linkerd-await", "--shutdown", "--"]
CMD ["/fip_api"]
