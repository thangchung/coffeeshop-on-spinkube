# coffeeshop-on-spinkube

## Prerequisites

`cargo-component` `spin v2.3.1` `wasm-tools` `k3d` `dapr` `spinkube`

## Install spinkube on k3d

```sh
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.14.3/cert-manager.yaml
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.runtime-class.yaml
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.crds.yaml
```

```sh
helm install spin-operator \
  --namespace spin-operator \
  --create-namespace \
  --version 0.1.0 \
  --wait \
  oci://ghcr.io/spinkube/charts/spin-operator
```

```sh
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.shim-executor.yaml
```

## Get starting

```sh
cd product-api
spin registry push --build ttl.sh/coffeeshop-product-api-spin:24h
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
