#!/bin/sh

## Create a k3d cluster
while (! kubectl cluster-info ); do
  # Docker takes a few seconds to initialize
  echo "Waiting for Docker to launch..."
  k3d cluster delete
  k3d cluster create wasm-cluster --image ghcr.io/spinkube/containerd-shim-spin/k3d:v0.13.0 --port '8081:80@loadbalancer' --agents 2 --k3s-arg '--disable=traefik@server:0'
  sleep 1
done

cargo install wasm-tools \
    && cargo install cargo-component \
    && cargo install cargo-shear

## dotnet
dotnet restore

## docker-compose
# docker compose -f ./docker-compose.yaml up -d