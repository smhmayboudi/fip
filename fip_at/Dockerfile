FROM docker.io/rust:1.56.0-bullseye as builder
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
RUN cargo build --package fip_at --release

FROM gcr.io/distroless/cc-debian11:nonroot
COPY --from=builder /fip/fip_at/at.db /
COPY --from=builder /fip/target/release/fip_at /
USER 65532
CMD ["/fip_at"]
