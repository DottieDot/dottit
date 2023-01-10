# Dottit

[![](https://github.com/dottiedot/Dottit/actions/workflows/authorization-service.yml/badge.svg)](https://github.com/DottieDot/dottit/actions/workflows/authorization-service.yml)
[![](https://github.com/dottiedot/Dottit/actions/workflows/board-service.yml/badge.svg)](https://github.com/DottieDot/dottit/actions/workflows/board-service.yml)
[![](https://github.com/dottiedot/Dottit/actions/workflows/comment-service.yml/badge.svg)](https://github.com/DottieDot/dottit/actions/workflows/comment-service.yml)
[![](https://github.com/dottiedot/Dottit/actions/workflows/gateway-service.yml/badge.svg)](https://github.com/DottieDot/dottit/actions/workflows/gateway-service.yml)
[![](https://github.com/dottiedot/Dottit/actions/workflows/mediator.yml/badge.svg)](https://github.com/DottieDot/dottit/actions/workflows/mediator.yml)
[![](https://github.com/dottiedot/Dottit/actions/workflows/thread-service.yml/badge.svg)](https://github.com/DottieDot/dottit/actions/workflows/thread-service.yml)
[![](https://github.com/dottiedot/Dottit/actions/workflows/user-service.yml/badge.svg)](https://github.com/DottieDot/dottit/actions/workflows/user-service.yml)


## Prerequisites
### Infrastructure Tools
* Docker
* Kubectl
* Krew
* Minikube
* Skaffold

### Language Tools
* Rust
* NodeJS
* NPM

### Database Tools
* Postgresql

## Getting started
### Setup Minikube VM/Kubernetes
#### Creation
To create the Minikube VM, run the following command:
```sh
minikube start --cpus=8 --memory 10240 --disk-size 192g
```

#### Enable ingress
Add the ingress addon to enable `ingress` deployments:
```sh
minikube addons enable ingress
```

#### Remove ingress validation
Remove the ingress validation step:
```sh
kubectl delete -A ValidatingWebhookConfiguration ingress-nginx-admission
```

### Install RabbitMQ kubectl plugin
```sh
kubectl krew install rabbitmq
```

#### Install RabbitMQ Cluster Operation
```sh
kubectl rabbitmq install-cluster-operator
```

### 

## Development

### RabbitMQ Web UI
To get the credentials to the web ui, run:
```sh
username="$(kubectl get secret message-bus-default-user -o jsonpath='{.data.username}' | base64 --decode)"
echo "username: $username"
password="$(kubectl get secret message-bus-default-user -o jsonpath='{.data.password}' | base64 --decode)"
echo "password: $password"
```

To open the RabbitMQ web UI, run:
```sh
kubectl rabbitmq manage message-bus
```

### Run services
To run a service and its dependencies, run:
```sh
skaffold dev --tolerate-failures-until-deadline  
```


```sh
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/controller-v1.5.1/deploy/static/provider/cloud/deploy.yaml
```