FROM rust:latest as build-env
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
# RUN mkdir .cargo
# RUN RUN cargo vendor > .cargo/config
RUN cargo build --release --package fip_user

FROM debian:stable-slim
RUN addgroup --gid 1000 fip
RUN adduser --gid 1000 --shell /bin/sh --uid 1000 fip
WORKDIR /fip_user
COPY --from=build-env /fip/fip_user/at.db .
COPY --from=build-env /fip/target/release/fip_user .
RUN chown fip:fip *
USER fip
CMD ["./fip_user"]
