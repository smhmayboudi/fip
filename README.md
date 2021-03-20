# rustup

```bash
$ curl https://sh.rustup.rs -sSf | sh -s
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

```bash
$ cargo install cargo-audit
$ cargo install cargo-cache
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

```bash
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

```bash
$ cargo install critcmp
$ cargo install flamegraph
$ cargo install grcov
$ cargo install mdbook
$ cargo install ripgrep
$ cargo install rustscan
$ cargo install sqlx-cli
```

cargo bench -- --save-baseline before
cargo bench -- --save-baseline change
critcmp before change

# HELP

cargo watch -x "run | bunyan"
catflap -- cargo watch -x "run | bunyan"

grpcurl \
    -d '{"username":"smhmayboudi","password":"smhmayboudi"}' \
    -import-path fip_at/proto \
    -import-path fip_api/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_auth.proto \
    127.0.0.1:50050 fip.api.auth.Auth/Login

grpcurl \
    -d '{"token":"E9E752B9-C342-4D3D-BDCC-13FA8E79FA0B"}' \
    -import-path fip_at/proto \
    -import-path fip_api/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_auth.proto \
    127.0.0.1:50050 fip.api.auth.Auth/Token

grpcurl \
    -H 'authorization: JWT eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IkFEODZENEQzLUEzNkMtNDkyNC04NDJDLTA0RjhCNzRBMjBDQiJ9.eyJhdWQiOiJmaXBfYXBpIiwiZXhwIjoxNjE2NDc3NTQyLCJpYXQiOjE2MTI4Nzc1NDIsImlzcyI6ImZpcF9hcGkiLCJqdGkiOiI3OEI3OEZGOC0wQTI4LTRCQUMtQTY4RC1DODdDQkI5N0FDRkYiLCJuYmYiOjE2MTI4Nzc1NDIsInN1YiI6IkU1RDg2RTFELTMxQTQtNDk2NC05ODgxLUYxNjBGQzVCMTA3MyJ9.PswZqhpcXNwnl8cTRDJQAgucxgFF5LluS2b6G6oeU3nQ78pcrJ8a-doK9V6b4F72Fvq4p0TeewXlFW6SZKMv4c0pfaH4R1969p_mC7DsLVT8ucmcPa6GYmnGKO9VhaOID0H1BYtJ7gL_pZlktudFf4foxpJkcstmfhuHZxDZbr3CZm52L2nwBcSMxMMyZgCUOVg4g_0QIYSom-05_IabsCN2rIe39pe0BH19-frERFXRm8jnJm5ubtU2Zz9ELyIulhwn0SwIIwoLlm09OPsX0txGEMcehSos8_oAuEsQUytKks6bXQ_UFCfsJRk8r6DexFl5igViuFIINozuYMIpPPbyQVBzNYheIiGOAPGDDqEA1m_frpRbZl0BKhjStXQdXldhGWARd3gCBKQctmPu-gDqt_NOTmx8LcbuXDKnM0ofAiQq6URTPac0XlPq8K3KXRXwrI-37JSAKxlliDlrYSN1yHFHhBh6ZOvCEmvhFb6rWcG_D2cUz9im33phlS0Q5krujvmKm66hS787LjiD7Tnpr_eK_4Syp7edg9W4TV-ueMDX853wsnp5BF25iiEtW_gXGgZZFQnYuHGbb92nS1M31RBvrkZO4lSnwZqfU0WvLweY5uOcD_AIh6MzzUjs-P2q1yiBgt5EyAYG6sBYZ-RP2U2iOUEnaJz-zvt9aVY' \
    -d '{"id":"E5D86E1D-31A4-4964-9881-F160FC5B1073"}' \
    -import-path fip_api/proto \
    -import-path fip_at/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_user.proto \
    127.0.0.1:50050 fip.api.user.User/FindOne

grpcurl \
    -H 'authorization: JWT eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IkFEODZENEQzLUEzNkMtNDkyNC04NDJDLTA0RjhCNzRBMjBDQiJ9.eyJhdWQiOiJmaXBfYXBpIiwiZXhwIjoxNjE2NDc3NTQyLCJpYXQiOjE2MTI4Nzc1NDIsImlzcyI6ImZpcF9hcGkiLCJqdGkiOiI3OEI3OEZGOC0wQTI4LTRCQUMtQTY4RC1DODdDQkI5N0FDRkYiLCJuYmYiOjE2MTI4Nzc1NDIsInN1YiI6IkU1RDg2RTFELTMxQTQtNDk2NC05ODgxLUYxNjBGQzVCMTA3MyJ9.PswZqhpcXNwnl8cTRDJQAgucxgFF5LluS2b6G6oeU3nQ78pcrJ8a-doK9V6b4F72Fvq4p0TeewXlFW6SZKMv4c0pfaH4R1969p_mC7DsLVT8ucmcPa6GYmnGKO9VhaOID0H1BYtJ7gL_pZlktudFf4foxpJkcstmfhuHZxDZbr3CZm52L2nwBcSMxMMyZgCUOVg4g_0QIYSom-05_IabsCN2rIe39pe0BH19-frERFXRm8jnJm5ubtU2Zz9ELyIulhwn0SwIIwoLlm09OPsX0txGEMcehSos8_oAuEsQUytKks6bXQ_UFCfsJRk8r6DexFl5igViuFIINozuYMIpPPbyQVBzNYheIiGOAPGDDqEA1m_frpRbZl0BKhjStXQdXldhGWARd3gCBKQctmPu-gDqt_NOTmx8LcbuXDKnM0ofAiQq6URTPac0XlPq8K3KXRXwrI-37JSAKxlliDlrYSN1yHFHhBh6ZOvCEmvhFb6rWcG_D2cUz9im33phlS0Q5krujvmKm66hS787LjiD7Tnpr_eK_4Syp7edg9W4TV-ueMDX853wsnp5BF25iiEtW_gXGgZZFQnYuHGbb92nS1M31RBvrkZO4lSnwZqfU0WvLweY5uOcD_AIh6MzzUjs-P2q1yiBgt5EyAYG6sBYZ-RP2U2iOUEnaJz-zvt9aVY' \
    -d '{"id":"E5D86E1D-31A4-4964-9881-F160FC5B1073"}' \
    -import-path fip_api/proto \
    -import-path fip_at/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_api.proto \
    127.0.0.1:50050 fip.api.api.Api/FindOne

# HUSKY

CARGO_HUSKY_DONT_INSTALL_HOOKS=true cargo test

# TLS

https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o

```bash
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

```bash
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
```
