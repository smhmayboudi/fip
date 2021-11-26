#!/bin/sh

set -eux
set -o errexit
set -o pipefail
set -o nounset

ref=${1:-"refs/heads/0-build"}
name=${ref#"refs/heads/"}

echo "::set-output name=name::${name}"
