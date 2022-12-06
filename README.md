# Dottit

## Prerequisites
### Infrastructure Tools
* Docker
* Kubectl
* Krew
* Minikube

### Language Tools
* Go
* Rust

### Database Tools
* Postgresql
* MySQL

## Getting started
### Setup Minikube VM/Kubernetes
#### Creation
To create the Minikube VM, run the following command:
```sh
minikube start --cpus=8 --memory 9965 --disk-size 96g
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
