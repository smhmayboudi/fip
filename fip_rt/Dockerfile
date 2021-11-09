FROM docker.io/rust:1.56.1-bullseye as builder
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
RUN cargo build --package fip_rt --release

FROM gcr.io/distroless/cc-debian11:nonroot
COPY --from=builder /fip/fip_rt/at.db /
COPY --from=builder /fip/target/release/fip_rt /
USER 65532
CMD ["/fip_rt"]
