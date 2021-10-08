#!/bin/bash

set -eux
set -o errexit
set -o pipefail
set -o nounset

COMMIT_SHA=${1:-""}
VERSION=${2:-"0.1.0"}

GIT_COMMITTER_DATE=$(git log -n1 --pretty=%aD $COMMIT_SHA) git tag -a -m "version $VERSION" "v$VERSION"
git push --tags
