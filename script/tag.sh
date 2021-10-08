#!/bin/sh

set -eux
set -o errexit
set -o pipefail
set -o nounset

VERSION=${VERSION:-0.1.0}
COMMIT_SHA=${COMMIT_SHA:-}

GIT_COMMITTER_DATE=$(git log -n1 --pretty=%aD $COMMIT_SHA) git tag -a -m "version $VERSION" "v$VERSION"
git push --tags
