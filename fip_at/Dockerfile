FROM rust:latest as build-env
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
# RUN mkdir .cargo
# RUN RUN cargo vendor > .cargo/config
RUN cargo build --release --package fip_at

FROM debian:stable-slim
RUN addgroup --gid 1000 fip
RUN adduser --gid 1000 --shell /bin/sh --uid 1000 fip
WORKDIR /fip_at
COPY --from=build-env /fip/fip_at/at.db .
COPY --from=build-env /fip/target/release/fip_at .
RUN chown fip:fip *
USER fip
CMD ["./fip_at"]
