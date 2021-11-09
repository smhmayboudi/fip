# # FROM docker.io/curlimages/curl:7.77.0 as linkerd
# # ARG LINKERD_AWAIT_VERSION=v0.2.4
# # RUN curl --location --output /tmp/linkerd-await --show-error --silent https://github.com/linkerd/linkerd-await/releases/download/release%2F${LINKERD_AWAIT_VERSION}/linkerd-await-${LINKERD_AWAIT_VERSION}-amd64 \
# #     && chmod 755 /tmp/linkerd-await

# FROM rust:1.56.1-bullseye as builder
# WORKDIR /fip_api
# ADD . .
# RUN rustup component add rustfmt
# RUN cargo build --release --package fip_api

# FROM gcr.io/distroless/cc-debian11:nonroot
# # COPY --from=linkerd /tmp/linkerd-await /linkerd-await
# COPY --from=builder /fip/fip_api/auth_model.conf /
# COPY --from=builder /fip/fip_api/auth_policy.csv /
# COPY --from=builder /fip/target/release/fip_api /
# USER 65532
# # ENTRYPOINT ["/linkerd-await", "--shutdown", "--"]
# CMD ["/fip_api"]

FROM gcr.io/distroless/static-debian11:nonroot
ARG VERSION="v0.1.0"
COPY ./auth_model.conf /
COPY ./auth_policy.csv /
RUN set -eux \
    && curl --location --output /fip_api --show-error --silent https://github.com/smhmayboudi/fip/releases/download/${VERSION}/fip_api-${VERSION}-aarch64 \
    && chmod 755 /fip_api
USER 65532
CMD ["/fip_api"]
