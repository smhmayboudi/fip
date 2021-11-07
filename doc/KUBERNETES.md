# Kubernetes

## Kubernetes Cluster Create

```shell
# 1st WAY
$ brew install kind
$ ./kind-with-registry.sh

# 2nd WAY
$ Docker for Desktop

# 3rd WAY
$ brew install ubuntu/microk8s/microk8s
$ microk8s install

# 4th WAY
$ brew cask install virtualbox
$ brew install minikube
$ minikube start

# 5th WAY
$ brew install k3d
```

## Kubernetes Cluster Working

```shell
$ brew install helm
$ brew install kube-ps1
$ brew install kubectx
$ brew install kubernetes-cli
```

## Kubernetes Cluster Working with UI

```shell
$ brew cask install lens
$ brew cask install kui
```

## Kubernetes Linkerd Service Mesh

```shell
$ brew install linkerd
$ curl https://kind.sigs.k8s.io/examples/kind-with-registry.sh > ./script/kind-with-registry.sh
$ chmod 777 ./script/kind-with-registry.sh
$ ./script/kind-with-registry.sh
 
# [https://linkerd.io/2.11/getting-started/]
$ kubectl version --short
$ curl -sSL https://run.linkerd.io/install | sh [BY PASS IF YOU USED BREW]
$ linkerd version
$ linkerd check --pre
$ linkerd install | kubectl apply -f -
$ linkerd check
$ linkerd viz install | kubectl apply -f -

## optional
$ linkerd jaeger install | kubectl apply -f -
$ linkerd multicluster install | kubectl apply -f -
$ linkerd check
$ linkerd viz dashboard &
$ linkerd -n linkerd-viz viz top deployment/web [ERROR]

$ docker build . -f ./fip_api/Dockerfile -t fip-api:0.1.0-nonroot
$ docker tag fip-api:0.1.0-nonroot 127.0.0.1:5000/fip-api:0.1.0-nonroot
$ docker push 127.0.0.1:5000/fip-api:0.1.0-nonroot
$ cat ./fip_api/fip-api.yml | kubectl apply -f -

$ kubectl -n fip-api-namespace port-forward service/fip-api 8080:8080

$ kubectl get -n fip-api-namespace deployment -o yaml | linkerd inject - | kubectl apply -f -
$ linkerd -n fip-api-namespace check --proxy
$ linkerd -n fip-api-namespace viz stat deployment
```
