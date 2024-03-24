# coffeeshop-on-spinkube

## Prerequisites

`cargo-component` `spin v2.3.1` `wasm-tools` `k3d` `dapr` `spinkube`

## Install spinkube on k3d

```sh
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.14.3/cert-manager.yaml
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.runtime-class.yaml
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.crds.yaml
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.shim-executor.yaml
```

```sh
helm install spin-operator \
  --namespace spin-operator \
  --create-namespace \
  --version 0.1.0 \
  --wait \
  oci://ghcr.io/spinkube/charts/spin-operator
```

## Install Dapr

```sh
> dapr --version
CLI version: 1.13.0 
Runtime version: n/a
```

```sh
# for demo only
> dapr init -k
```

```sh
> helm install my-redis oci://registry-1.docker.io/bitnamicharts/redis --set architecture=standalone --set global.redis.password=P@ssw0rd
```

```sh
> kubectl apply -f deploys/dapr/
> kubectl get components
```

## Build and push apps

```sh
> cargo component new --lib counter-api
> cd counter-api
> cargo component build --release
```

```sh
cd product-api
spin registry push --build ttl.sh/coffeeshop-product-api-spin:24h
```

## Deploy SpinApp

```sh
> kubectl apply -f deploys/spinapp-product-api.yaml
> kubectl get po
```

```sh
> kubectl apply -f deploys/ingress.yaml
```

### Dapr Shared mode

```sh
> helm install product-api-shared oci://docker.io/daprio/dapr-shared-chart --set shared.appId=product-api --set shared.remoteURL=product-api --set shared.remotePort=5001
```

> Notes: Your `spin-operator.shim-executor.yaml` should be apply on the same Kubernetes's `namespace` of the service you deploy.

## Fixing un-used packages

```sh
> cargo shear --fix
```

## References

- https://www.spinkube.dev/docs/spin-operator/quickstart/
- https://github.com/dapr-sandbox/dapr-shared
- https://github.com/bytecodealliance/cargo-component
