FROM docker.io/curlimages/curl:7.80.0 as fip_jwks
ARG BUILDARCH
ARG BUILDOS
ARG BUILDPLATFORM
ARG BUILDVARIANT
ARG TARGETARCH
ARG TARGETOS
ARG TARGETPLATFORM
ARG TARGETVARIANT
ARG VERSION
RUN set -eux \
    && curl --location --output /tmp/fip_jwks --show-error --silent https://github.com/smhmayboudi/fip/releases/download/v${VERSION}/fip_jwks-v${VERSION}-${TARGETARCH}${TARGETVARIANT} \
    && chmod 755 /tmp/fip_jwks

# FROM docker.io/curlimages/curl:7.80.0 as linkerd-await
# ARG LINKERD_AWAIT_VERSION=v0.2.4
# RUN curl --location --output /tmp/linkerd-await --show-error --silent https://github.com/linkerd/linkerd-await/releases/download/release%2F${LINKERD_AWAIT_VERSION}/linkerd-await-${LINKERD_AWAIT_VERSION}-amd64 \
#     && chmod 755 /tmp/linkerd-await

FROM gcr.io/distroless/static-debian11:nonroot
COPY --chown=nonroot:nonroot --from=fip_jwks /tmp/fip_jwks /fip_jwks
# COPY --chown=nonroot:nonroot --from=linkerd-await /tmp/linkerd-await /linkerd-await
COPY --chown=nonroot:nonroot ./jwks.db /
USER nonroot:nonroot
# ENTRYPOINT ["/linkerd-await", "--shutdown", "--"]
CMD ["/fip_jwks"]
