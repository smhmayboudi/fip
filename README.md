[![Build Status](__badge_image__)](__badge_url__)

# rustup

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
$ rustup toolchain install nightly
$ rustup component add \
    cargo \
    clippy \
    llvm-tools-preview \
    rls \
    rust-analysis \
    rust-docs \
    rust-src \
    rust-std \
    rustc \
    rustfmt
$ rustup +nightly component add miri
$ rustup update && rustup self update
```

# Cargo

## Sub Commands

### Install

```shell
$ cargo install cargo-audit
$ cargo install cargo-cache
$ cargo install cargo-deny
$ cargo install cargo-diet
$ cargo install cargo-edit
$ cargo install cargo-expand
$ cargo install cargo-inspect
$ cargo install cargo-make
$ cargo install cargo-modules
$ cargo install cargo-outdated
$ cargo install cargo-readme
$ cargo install cargo-spellcheck
$ cargo install cargo-watch
```

### Run

```shell
$ cargo bench
$ cargo cache --autoclean
$ cargo check
$ cargo doc
$ cargo fix
$ cargo inspect
$ cargo package
$ cargo publish
$ cargo readme > README.md
$ cargo run
$ cargo rustc
$ cargo spellcheck check
$ cargo spellcheck fix
$ cargo test
$ cargo update
$ cargo upgrade --workspace
$ cargo vendor
```

## Tools

```shell
$ cargo install bunyan
$ cargo install critcmp
$ cargo install flamegraph
$ cargo install grcov
$ cargo install mdbook
$ cargo install ripgrep
$ cargo install rustscan
$ cargo install sqlx-cli
```

```shell
$ cargo bench -- --save-baseline before
$ cargo bench -- --save-baseline change
$ critcmp before change
```

# HELP

```shell
$ cargo watch -x "run | bunyan"
$ catflap -- cargo watch -x "run | bunyan"

$ grpcurl \
    -d '{"username":"smhmayboudi","password":"smhmayboudi"}' \
    -import-path fip_at/proto \
    -import-path fip_api/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_auth.proto \
    127.0.0.1:50050 fip.api.auth.Auth/Login

$ grpcurl \
    -d '{"token":"E9E752B9-C342-4D3D-BDCC-13FA8E79FA0B"}' \
    -import-path fip_at/proto \
    -import-path fip_api/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_auth.proto \
    127.0.0.1:50050 fip.api.auth.Auth/Token

$ grpcurl \
    -H 'authorization: JWT eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IkFEODZENEQzLUEzNkMtNDkyNC04NDJDLTA0RjhCNzRBMjBDQiJ9.eyJhdWQiOiJmaXBfYXBpIiwiZXhwIjoxNjI0MzMyMDY0LCJpYXQiOjE2MjA3MzIwNjQsImlzcyI6ImZpcF9hcGkiLCJqdGkiOiI0RDI5OTBGRC04OEY2LTQ1QTEtQTkwRS1ENzU5OENCQkRCMzQiLCJuYmYiOjE2MjA3MzIwNjQsInN1YiI6IkU1RDg2RTFELTMxQTQtNDk2NC05ODgxLUYxNjBGQzVCMTA3MyJ9.Rrrt0mqL827Olh9aaLPL4mp3RRv_CGsCdVqN6kbrNLvbczi88l72zIXvk832U6Nyu0xY0sOlkxKeu9gA4PhUcX8x7wfZWbTLY5YJHa6LWl067gVdItW7toU6YqRyCe9S33yNMD5x2UNkdYDI1QA-l0M2SuuQQnP1I_At27Z2NZsX9sQVt9K59xtBOM00TmVjleWjx5Dw357pTulbWGSafDPOWpY1ahW6UWnY0F4Y4Ah20pWCD04Ind4ioXobONt5w0YLAw56dkbIJFrTKqR4JbfcvySmTDxlWrKbYIF3UcW8FZl1017hlFgUHRtg0JOy3yputuR9NVl3D_qerv0aQfSH6qiJ8d2yiPDrbEuNWp3pa2cPXCAZ5BSXysxIvyYSC6i8bh-dzGUesB3dDNc2EWB76AUo8208Jdq1BoO7g9321ReDJvuymp031wMFfBdxPgRRQbsuodbVaWtVH9JjAL3YFqAKq-6AT-aux0nYSqob4AzbbEy1mdyl0jm18ahmOKWzNT_yfD6IaEV3dlT6y9w-HSxenK6ETlVsaLTwS-qd0jZAm9tiiFwqbgVk7EIIN4ut1m-KeP6T3e38JQdtzaec0g4nLUQ0wAfLOlA2F4mNukvEL9TMOdpDd0RFQWTmHVuXcPOD0ykBunurL2T2jnDWXNEsF9MnSjQl0sLZies' \
    -d '{"id":"E5D86E1D-31A4-4964-9881-F160FC5B1073"}' \
    -import-path fip_api/proto \
    -import-path fip_at/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_user.proto \
    127.0.0.1:50050 fip.api.user.User/FindOne

$ grpcurl \
    -H 'authorization: JWT eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IkFEODZENEQzLUEzNkMtNDkyNC04NDJDLTA0RjhCNzRBMjBDQiJ9.eyJhdWQiOiJmaXBfYXBpIiwiZXhwIjoxNjI0MzMyMDY0LCJpYXQiOjE2MjA3MzIwNjQsImlzcyI6ImZpcF9hcGkiLCJqdGkiOiI0RDI5OTBGRC04OEY2LTQ1QTEtQTkwRS1ENzU5OENCQkRCMzQiLCJuYmYiOjE2MjA3MzIwNjQsInN1YiI6IkU1RDg2RTFELTMxQTQtNDk2NC05ODgxLUYxNjBGQzVCMTA3MyJ9.Rrrt0mqL827Olh9aaLPL4mp3RRv_CGsCdVqN6kbrNLvbczi88l72zIXvk832U6Nyu0xY0sOlkxKeu9gA4PhUcX8x7wfZWbTLY5YJHa6LWl067gVdItW7toU6YqRyCe9S33yNMD5x2UNkdYDI1QA-l0M2SuuQQnP1I_At27Z2NZsX9sQVt9K59xtBOM00TmVjleWjx5Dw357pTulbWGSafDPOWpY1ahW6UWnY0F4Y4Ah20pWCD04Ind4ioXobONt5w0YLAw56dkbIJFrTKqR4JbfcvySmTDxlWrKbYIF3UcW8FZl1017hlFgUHRtg0JOy3yputuR9NVl3D_qerv0aQfSH6qiJ8d2yiPDrbEuNWp3pa2cPXCAZ5BSXysxIvyYSC6i8bh-dzGUesB3dDNc2EWB76AUo8208Jdq1BoO7g9321ReDJvuymp031wMFfBdxPgRRQbsuodbVaWtVH9JjAL3YFqAKq-6AT-aux0nYSqob4AzbbEy1mdyl0jm18ahmOKWzNT_yfD6IaEV3dlT6y9w-HSxenK6ETlVsaLTwS-qd0jZAm9tiiFwqbgVk7EIIN4ut1m-KeP6T3e38JQdtzaec0g4nLUQ0wAfLOlA2F4mNukvEL9TMOdpDd0RFQWTmHVuXcPOD0ykBunurL2T2jnDWXNEsF9MnSjQl0sLZies' \
    -d '{"id":"E5D86E1D-31A4-4964-9881-F160FC5B1073"}' \
    -import-path fip_api/proto \
    -import-path fip_at/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_api.proto \
    127.0.0.1:50050 fip.api.api.Api/FindOne
```

# HUSKY

```shell
$ CARGO_HUSKY_DONT_INSTALL_HOOKS=true cargo test
```

# TLS

https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o

```shell
$ openssl genrsa -des3 -out my_ca.key 2048
$ openssl req -x509 -new -nodes -key my_ca.key -sha256 -days 1825 -out my_ca.pem
$ openssL genrsa -out server.key 2048
$ openssl req -new -sha256 -key server.key -out server.csr
# authorityKeyIdentifier=keyid,issuer
# basicConstraints=CA:FALSE
# keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
# subjectAltName = @alt_names
# 
# [alt_names]
# DNS.1 = localhost
$ openssl x509 -req -in server.csr -CA my_ca.pem -CAkey my_ca.key -CAcreateserial -out server.pem -days 1825 -sha256 -extfile server.ext
```

# Jaeger Tracing

```shell
$ docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:1.23.0
```
