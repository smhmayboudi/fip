FROM docker.io/rust:1.55.0-buster as builder
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
# RUN mkdir .cargo
# RUN RUN cargo vendor > .cargo/config
RUN cargo build --package fip_jwks --release

FROM gcr.io/distroless/cc:nonroot
COPY --from=builder /fip/fip_jwks/at.db /
COPY --from=builder /fip/target/release/fip_jwks /
USER 65532
CMD ["/fip_jwks"]
