# Cross Build

```shell
# 1st WAY
$ make release CARGO="cross" PACKAGE="fip_api" RELEASE="--release" STRIP="strip" TARGET="x86_64-unknown-linux-musl" VERSION="$(git describe --tags --abbrev=0)"

# 2nd WAY
$ docker build . -t rust-build:1.54.0-buster
$ docker run -v $(pwd):/project -v /var/run/docker.sock:/var/run/docker.sock -w /project --rm rust-build:1.54.0-buster make release CARGO="cross" PACKAGE="fip_api" RELEASE="--release" STRIP="strip" TARGET="x86_64-unknown-linux-musl" VERSION="$(git describe --tags --abbrev=0)"
```
