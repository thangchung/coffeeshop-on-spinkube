# Step by step

## AKS setup

```sh
az login --use-device-code
az account set --subscription "fd2f0c4b-23b2-4fce-8c33-f4e0de4a0510"
az account show
```

```sh
az group create --name globalazure24 --location southeastasia

az aks create \
--resource-group globalazure24 \
--name kwasm \
--node-count 1

az aks get-credentials --resource-group globalazure24 --name kwasm

# create another nodepool - user mode and Azure spot instance

az aks nodepool update -g globalazure24 --cluster-name kwasm -n nodepool1 --node-taints CriticalAddonsOnly=true:NoSchedule --mode system

# IMPORTANT: then run kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.shim-executor.yaml should work, don't know why???

az aks nodepool update -g globalazure24 --cluster-name kwasm -n nodepool1 --node-taints "" --mode system

# IMPORTANT: then remove tants node, now we can run normally, dont' know why???

az k8s-extension create --cluster-type managedClusters --cluster-name kwasm --resource-group globalazure24 --name dapr --extension-type Microsoft.Dapr
```

## SpinKube installation

```sh
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.14.4/cert-manager.crds.yaml
```

Install `kwasm`, so we can have `containerd-shim-spin`

```sh
# Add Helm repositories jetstack and KWasm
helm repo add jetstack https://charts.jetstack.io
helm repo add kwasm http://kwasm.sh/kwasm-operator

# Update Helm repositories
helm repo update
```

```sh
# Install cert-manager using Helm
helm install \
  cert-manager jetstack/cert-manager \
  --namespace cert-manager \
  --create-namespace \
  --version v1.14.4
```

```sh
# Install KWasm operator
helm install \
  kwasm-operator kwasm/kwasm-operator \
  --namespace kwasm \
  --create-namespace \
  --set kwasmOperator.installerImage=ghcr.io/spinkube/containerd-shim-spin/node-installer:v0.13.1
```

```sh
kubectl annotate node aks-nodepool2-17015409-vmss000000 kwasm.sh/kwasm-node=true
kubectl annotate node --all kwasm.sh/kwasm-node=true-

# after tant node for  spin-operator.shim-executor.yaml, then it worked. Don't know why???
kubectl annotate node aks-nodepool1-14433946-vmss000000 kwasm.sh/kwasm-node=true

# az aks nodepool start --resource-group globalazure24 --cluster-name kwasm --nodepool-name nodepool1
```

```sh
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.runtime-class.yaml
```

```sh
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.crds.yaml
```

```sh
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.shim-executor.yaml
```

## Create SpinApp

```sh
cd product-api
export IMAGE_NAME=ttl.sh/coffeeshop-product-api-spin:24h
spin registry push --build $IMAGE_NAME
spin kube scaffold -f $IMAGE_NAME > app.yaml
kubectl apply -f ../deploys/spinapp-product-api.yaml
```

## Create SpinApp with Dapr

```sh
helm install my-redis oci://registry-1.docker.io/bitnamicharts/redis --set architecture=standalone --set global.redis.password=P@ssw0rd
kubectl apply -f deploys/dapr/
kubectl get components

# un-comment daprd lines on deploys/spinapp-product-api.yaml
kubectl apply -f deploys/spinapp-product-api.yaml
kubectl apply -f deploys/ingress.yaml
```

## Troubleshooting

When you run `kubectl get po`, and get something like `E0418 09:09:32.063683   89871 memcache.go:265] couldn't get current server API group list: Get "https://kwasm-azurebootcamp202-fd2f0c-s6umj6p0.hcp.southeastasia.azmk8s.io:443/api?timeout=32s": tls: failed to verify certificate: x509: certificate has expired or is not yet valid: current time 2024-04-18T09:09:32Z is before 2024-04-18T14:41:19Z`. Then run command below

```sh
# ref https://github.com/microsoft/WSL/issues/4245#issuecomment-510238198
sudo hwclock -s
```

```sh
kubectl get events --sort-by='.lastTimestamp' --all-namespaces
```

## TODO

```sh
# TODO: not work for southeastasia
az aks nodepool add \
  --resource-group globalazure24 \
  --cluster-name kwasm \
  --name nodepool2 \
  --priority Spot \
  --eviction-policy Delete \
  --spot-max-price -1 \
  --enable-cluster-autoscaler \
  --min-count 1 \
  --max-count 3 \
  --mode user \
  --os-sku Ubuntu \
  --no-wait
```
