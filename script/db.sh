#!/bin/bash

set -eux
set -o errexit
set -o pipefail
set -o nounset

PACKAGES="api at jwks rt user"

for PACKAGE in ${PACKAGES}; do
    cd "./fip_${PACKAGE}"
    rm -fr "${PACKAGE}.db*"
    sqlx database create --database-url "sqlite:${PACKAGE}.db"
    sqlx migrate run --database-url "sqlite:${PACKAGE}.db"
    cd ..
done
