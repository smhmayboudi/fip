# Extra Command

```shell
$ rustup target add \
    aarch64-unknown-linux-musl \
    armv7-unknown-linux-musleabihf \
    x86_64-unknown-linux-musl
$ cargo build --package fip_api --release --target x86_64-unknown-linux-musl

$ docker build . -f ./fip_api/Dockerfile -t fip-api:0.1.0-nonroot
$ docker run -e LINKERD_AWAIT_DISABLED=TRUE -i -p 8080:8080 --rm fip-api:0.1.0-nonroot

$ shasum -a 256 target/release/fip_api

$ brew install upx
$ upx --ultra-brute ./target/x86_64-unknown-linux-musl/release/fip_api
```

```shell
$ GIT_COMMITTER_DATE=$(git log -n1 --pretty=%aD) git tag -a -m "Release 0.1.0" 0.1.0
$ git push --tags

$ rustc --print target-list
$ rustc --target ${TRIPLE} --print target-cpus
$ rustc --target ${TRIPLE} --print target-features

$ perf record --call-graph=dwarf ./target/release/fip_api
$ perf report --hierarchy -M intel

$ RUSTFLAGS="-C target-cpu=native" cargo build --release
```

```shell
$ export CARGO_INCREMENTAL=0
$ export RUSTDOCFLAGS="-Cpanic=abort"
$ export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
```
