#!/bin/sh

set -eux
set -o errexit
set -o pipefail
set -o nounset

version=0.1.0
GIT_COMMITTER_DATE=$(git log -n1 --pretty=%aD) git tag -a -m "version $version" "v$version"
git push --tags
