# Cross Build

```shell
# 1st WAY
$ make release CARGO="cross" PACKAGE="fip_api" RELEASE="--release" STRIP="aarch64-linux-gnu-strip" TARGET="aarch64-unknown-linux-musl" VERSION="$(git describe --tags --abbrev=0)"

# run before run the rest 
$ docker build . -t rust-build:1.56.0-bullseye

# 2nd WAY
$ docker run -v $(pwd):/workspace -v /var/run/docker.sock:/var/run/docker.sock -w /workspace --rm rust-build:1.56.0-bullseye make release CARGO="cross" PACKAGE="fip_api" RELEASE="--release" STRIP="aarch64-linux-gnu-strip" TARGET="aarch64-unknown-linux-musl" VERSION="$(git describe --tags --abbrev=0)"

# 2nd WAY: devcontainer docker-from-docker 
$ docker run -v ${LOCAL_WORKSPACE_FOLDER}:/workspace -w /workspace --rm rust-build:1.56.0-bullseye make release CARGO="cross" PACKAGE="fip_api" RELEASE="--release" STRIP="aarch64-linux-gnu-strip" TARGET="aarch64-unknown-linux-musl" VERSION="$(git describe --tags --abbrev=0)"
```
