FROM docker.io/curlimages/curl:7.80.0 as fip_user
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
    && curl --location --output /tmp/fip_user --show-error --silent https://github.com/smhmayboudi/fip/releases/download/v${VERSION}/fip_user-v${VERSION}-${TARGETARCH}${TARGETVARIANT} \
    && chmod 755 /tmp/fip_user

# FROM docker.io/curlimages/curl:7.80.0 as linkerd-await
# ARG LINKERD_AWAIT_VERSION=v0.2.4
# RUN curl --location --output /tmp/linkerd-await --show-error --silent https://github.com/linkerd/linkerd-await/releases/download/release%2F${LINKERD_AWAIT_VERSION}/linkerd-await-${LINKERD_AWAIT_VERSION}-amd64 \
#     && chmod 755 /tmp/linkerd-await

FROM gcr.io/distroless/static-debian11:nonroot
COPY --chown=nonroot:nonroot --from=fip_user /tmp/fip_user /fip_user
# COPY --chown=nonroot:nonroot --from=linkerd-await /tmp/linkerd-await /linkerd-await
COPY --chown=nonroot:nonroot ./user.db /
USER nonroot:nonroot
# ENTRYPOINT ["/linkerd-await", "--shutdown", "--"]
CMD ["/fip_user"]
