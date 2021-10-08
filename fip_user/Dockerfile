FROM docker.io/rust:1.55.0-buster as builder
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
# RUN mkdir .cargo
# RUN RUN cargo vendor > .cargo/config
RUN cargo build --package fip_user --release

FROM gcr.io/distroless/cc:nonroot
COPY --from=builder /fip/fip_user/at.db /
COPY --from=builder /fip/target/release/fip_user /
USER 65532
CMD ["/fip_user"]
