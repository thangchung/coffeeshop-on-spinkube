#!/bin/sh

# # spinkube 
# kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.14.3/cert-manager.yaml
# kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.runtime-class.yaml
# kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.crds.yaml

# helm install spin-operator \
#   --namespace spin-operator \
#   --create-namespace \
#   --version 0.1.0 \
#   --wait \
#   oci://ghcr.io/spinkube/charts/spin-operator

# kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.shim-executor.yaml

# # Dapr
# ## for development only
# dapr init -k

# helm install my-redis oci://registry-1.docker.io/bitnamicharts/redis --set architecture=standalone --set global.redis.password=P@ssw0rd