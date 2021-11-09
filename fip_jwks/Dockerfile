FROM docker.io/rust:1.56.1-bullseye as builder
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
RUN cargo build --package fip_jwks --release

FROM gcr.io/distroless/cc-debian11:nonroot
COPY --from=builder /fip/fip_jwks/at.db /
COPY --from=builder /fip/target/release/fip_jwks /
USER 65532
CMD ["/fip_jwks"]
