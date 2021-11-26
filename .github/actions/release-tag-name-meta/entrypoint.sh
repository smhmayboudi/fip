#!/bin/sh

set -eux
set -o errexit
set -o pipefail
set -o nounset

ref=${1:-"v0.1.0"}
version=${ref#"v"}
if [ -z "${ref##*'rc'*}" ] ;then
    prerelease=true
else
    prerelease=false
fi

echo "::set-output name=name::version ${version}"
echo "::set-output name=prerelease::${prerelease}"
echo "::set-output name=version::${version}"
