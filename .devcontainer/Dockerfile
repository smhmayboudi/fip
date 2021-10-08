# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.202.5/containers/rust/.devcontainer/base.Dockerfile

ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0.201.0-1-${VARIANT}

RUN export DEBIAN_FRONTEND=noninteractive \
    && apt-get update \
    && apt-get -y install --no-install-recommends cmake git sqlite3
