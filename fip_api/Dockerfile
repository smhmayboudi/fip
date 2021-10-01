FROM docker.io/rust:1.53.0-buster as builder
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
# RUN mkdir .cargo
# RUN RUN cargo vendor > .cargo/config
RUN cargo build  --package fip_api --release

FROM gcr.io/distroless/cc:nonroot
COPY --from=builder /fip/fip_api/auth_model.conf /
COPY --from=builder /fip/fip_api/auth_policy.csv /
COPY --from=builder /fip/target/release/fip_api /
USER 65532
CMD ["/fip_api"]
