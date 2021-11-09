#!/bin/sh

set -eux
set -o errexit
set -o pipefail
set -o nounset

ref=${1:-"refs/tags/v0.1.0"}
name=$(echo "${ref}" | sed s,^refs/tags/v,,)
tag=$(echo "${ref}" | sed s,^refs/tags/,,)

echo ::set-output "name=name::${name}"
echo ::set-output "name=tag::${tag}"
