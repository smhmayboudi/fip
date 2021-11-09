FROM docker.io/rust:1.56.1-bullseye as builder
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
RUN cargo build --package fip_user --release

FROM gcr.io/distroless/cc-debian11:nonroot
COPY --from=builder /fip/fip_user/at.db /
COPY --from=builder /fip/target/release/fip_user /
USER 65532
CMD ["/fip_user"]
