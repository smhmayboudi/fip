FROM rust:latest as build-env
WORKDIR /fip
ADD . .
RUN rustup component add rustfmt
# RUN mkdir .cargo
# RUN RUN cargo vendor > .cargo/config
RUN cargo build --release --package fip_api

FROM debian:stable-slim
RUN addgroup --gid 1000 fip
RUN adduser --gid 1000 --shell /bin/sh --uid 1000 fip
WORKDIR /fip_api
COPY --from=build-env /fip/fip_api/auth_model.conf .
COPY --from=build-env /fip/fip_api/auth_policy.csv .
COPY --from=build-env /fip/target/release/fip_api .
RUN chown fip:fip *
USER fip
CMD ["./fip_api"]
