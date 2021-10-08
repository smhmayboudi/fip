FROM docker.io/rust:1.55.0-buster as builder
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
# RUN mkdir .cargo
# RUN RUN cargo vendor > .cargo/config
RUN cargo build --package fip_at --release

FROM gcr.io/distroless/cc:nonroot
COPY --from=builder /fip/fip_at/at.db /
COPY --from=builder /fip/target/release/fip_at /
USER 65532
CMD ["/fip_at"]
