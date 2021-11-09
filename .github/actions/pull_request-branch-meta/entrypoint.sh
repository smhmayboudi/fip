#!/bin/sh

set -eux
set -o errexit
set -o pipefail
set -o nounset

ref=${1:-"refs/heads/0-build"}
name=$(echo "${ref}" | sed s,^refs/heads/,,)

echo ::set-output "name=name::${name}"
