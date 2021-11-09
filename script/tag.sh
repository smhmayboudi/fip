#!/bin/bash

set -eux
set -o errexit
set -o pipefail
set -o nounset

COMMIT_SHA=${1:-""}
VERSION=${2:-"0.1.0"}

GIT_COMMITTER_DATE=$(git log --format=%aD --max-count=1 ${COMMIT_SHA}) git tag -a -m "version ${VERSION}" "v${VERSION}"
git push --tags --no-verify
