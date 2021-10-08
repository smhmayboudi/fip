# Docker Content Trust

```shell
$ docker trust key generate {NAME}

# LOAD
$ docker trust key load {NAME}.pem --name {NAME}

$ docker trust signer add --key {NAME}.pem {NAME} localhost:5000/fip-api
$ docker trust sign localhost:5000/fip-api:0.1.0-nonroot
$ export DOCKER_CONTENT_TRUST=1
$ docker push localhost:5000/fip-api:0.1.0-nonroot

# TEST
$ docker trust inspect --pretty localhost:5000/fip-api:0.1.0-nonroot

# REMOVE
$ docker trust revoke localhost:5000/fip-api:0.1.0-nonroot
```
