FROM docker.io/rust:1.55.0-buster as builder
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
RUN cargo build --package fip_rt --release

FROM gcr.io/distroless/cc:nonroot
COPY --from=builder /fip/fip_rt/at.db /
COPY --from=builder /fip/target/release/fip_rt /
USER 65532
CMD ["/fip_rt"]
