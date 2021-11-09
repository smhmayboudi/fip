#!/bin/bash

set -eux
set -o errexit
set -o pipefail
set -o nounset

# docker run --detach --publish "127.0.0.1:5000:5000" --restart always --name registry registry:2.7.1
# docker pull vsc-fip
# docker image tag vsc-fip 127.0.0.1:5000/vsc-fip
# docker push 127.0.0.1:5000/vsc-fip
# docker pull 127.0.0.1:5000/vsc-fip
# docker container stop registry && docker container rm --volumes registry
