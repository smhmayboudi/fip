# https://github.com/microsoft/vscode-dev-containers/blob/v0.205.1/containers/rust/.devcontainer/base.Dockerfile

ARG VARIANT=1.56.0-bullseye
FROM rust:${VARIANT}

ARG ADD_NON_FREE_PACKAGES="true"
ARG CARGO_HOME="/usr/local/cargo"
ARG DOCKER_VERSION="latest"
ARG ENABLE_NONROOT_DOCKER="true"
ARG FIX_ENVIRONMENT="true"
ARG GIT_VERSION="latest"
ARG HELM_SHA256="automatic"
ARG HELM_VERSION="latest"
ARG INSTALL_OH_MYS="true"
ARG INSTALL_ZSH="true"
ARG KUBECTL_SHA256="automatic"
ARG KUBECTL_VERSION="latest"
ARG MINIKUBE_SHA256="automatic"
ARG MINIKUBE_VERSION="none"
ARG NEW_PASSWORD="vscode"
ARG RUSTUP_HOME="/usr/local/rustup"
ARG RUSTUP_PROFILE="default"
ARG RUST_VERSION="latest"
ARG SOURCE_SOCKET="/var/run/docker-host.sock"
ARG SSHD_PORT="2222"
ARG START_SSHD="true"
ARG TARGET_SOCKET="/var/run/docker.sock"
ARG UPDATE_RC="true"
ARG UPDATE_RUST="true"
ARG UPGRADE_PACKAGES="true"
ARG USERNAME="vscode"
ARG USER_GID="${USER_UID}"
ARG USER_UID="1000"
ARG USE_MOBY="true"
ARG USE_PPA_IF_AVAILABLE="latest"

COPY library-scripts/*.sh library-scripts/*.env /tmp/library-scripts/

RUN export DEBIAN_FRONTEND=noninteractive \
    && apt-get update  \
    # Remove imagemagick due to https://security-tracker.debian.org/tracker/CVE-2019-10131
    && apt-get purge -y imagemagick imagemagick-6-common \
    && bash /tmp/library-scripts/common-debian.sh "${INSTALL_ZSH}" "${USERNAME}" "${USER_UID}" "${USER_GID}" "${UPGRADE_PACKAGES}" "${INSTALL_OH_MYS}" "${ADD_NON_FREE_PACKAGES}" \
    && bash /tmp/library-scripts/docker-debian.sh "${ENABLE_NONROOT_DOCKER}" "${SOURCE_SOCKET}" "${TARGET_SOCKET}" "${USERNAME}" "${USE_MOBY}" "${DOCKER_VERSION}" \
    # && bash /tmp/library-scripts/docker-in-docker-debian.sh "${ENABLE_NONROOT_DOCKER}" "${USERNAME}" "${USE_MOBY}" "${DOCKER_VERSION}" \
    && bash /tmp/library-scripts/git-from-src-debian.sh "${GIT_VERSION}" "${USE_PPA_IF_AVAILABLE}" \
    && bash /tmp/library-scripts/kubectl-helm-debian.sh "${KUBECTL_VERSION}" "${HELM_VERSION}" "${MINIKUBE_VERSION}" "${KUBECTL_SHA256}" "${HELM_SHA256}" "${MINIKUBE_SHA256}" \
    && bash /tmp/library-scripts/rust-debian.sh "${CARGO_HOME}" "${RUSTUP_HOME}" "${USERNAME}" "${UPDATE_RC}" "${UPDATE_RUST}" "${RUST_VERSION}" "${RUSTUP_PROFILE}" \
    && bash /tmp/library-scripts/sshd-debian.sh "${SSHD_PORT}" "${USERNAME}" "${START_SSHD}" "${NEW_PASSWORD}" "${FIX_ENVIRONMENT}" \
    && apt-get install -y --no-install-recommends binutils-aarch64-linux-gnu binutils-arm-linux-gnueabihf binutils-x86-64-linux-gnu cmake docker.io musl-tools jq sqlite3 \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /tmp/* /var/lib/apt/lists/* /var/tmp/*

COPY copy-kube-config.sh /usr/local/share/

RUN chown ${USERNAME}:root /usr/local/share/copy-kube-config.sh \
    && echo "source /usr/local/share/copy-kube-config.sh" | tee -a /root/.bashrc /root/.zshrc /home/${USERNAME}/.bashrc >> /home/${USERNAME}/.zshrc

ENV CROSS_DOCKER_IN_DOCKER=true
USER ${USERNAME}
RUN cargo install --locked cross
USER root

ENTRYPOINT ["/usr/local/share/docker-init.sh", "/usr/local/share/ssh-init.sh"]

CMD ["sleep", "infinity"]
