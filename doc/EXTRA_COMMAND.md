# Extra Command

```shell
$ rustup target add \
    aarch64-unknown-linux-musl \
    armv7-unknown-linux-musleabihf \
    x86_64-unknown-linux-musl
$ cargo build --package fip_api --release --target aarch64-unknown-linux-musl

$ docker build . --file ./fip_api/Dockerfile --tag fip-api:0.1.0-nonroot
$ docker run --env LINKERD_AWAIT_DISABLED=TRUE --interactive --publish 8080:8080 --rm fip-api:0.1.0-nonroot

$ shasum --algorithm 256 target/release/fip_api

$ brew install upx
$ upx --ultra-brute ./target/aarch64-unknown-linux-musl/release/fip_api
```

```shell
$ GIT_COMMITTER_DATE=$(git log --format=%aD --max-count=1) git tag -a -m "Release 0.1.0" 0.1.0
$ git push --tags

$ rustc --print target-list
$ rustc --print target-cpus --target ${TRIPLE}
$ rustc --print target-features --target ${TRIPLE}

$ perf record --call-graph dwarf ./target/release/fip_api
$ perf report --disassembler-style intel --hierarchy

$ RUSTFLAGS="-Ctarget-cpu=native" cargo build --release
```

```shell
$ export CARGO_INCREMENTAL=0
$ export RUSTDOCFLAGS="-Cpanic=abort"
$ export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
```
