# Use xtunnel in container

We push the OCI-based image to [Github Container Registry](https://ghcr.io) with name: `ghcr.io/zhboner/xtunnel`.

These are some tag of this image:

- `latest`, `v1.*` base on debian:bullseye-silm, recommend
- `alpine`, `v1.*-alpine` base on alpine:latest

## Docker

```bash
docker run -d -p 9000:9000 ghcr.io/zhboner/xtunnel:latest -l 0.0.0.0:9000 -r 192.168.233.2:9000
```

## Docker Swarm (Docker Compose)

```yaml
# ./xtunnel.yml
version: '3'
services:
  port-9000:
    image: ghcr.io/zhboner/xtunnel:latest
    ports:
      - 9000:9000
    command: -l 0.0.0.0:9000 -r 192.168.233.2:9000
```

```bash
docker-compose -f ./xtunnel.yml -p xtunnel up -d
```

## Kubernetes

```yaml
# ./xtunnel.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: xtunnel-demo-deployment
  labels:
    app: xtunnel
  namespace: default
spec:
  replicas: 1
  selector:
    matchLabels:
      app: xtunnel 
  template:
    metadata:
      labels:
        app: xtunnel 
    spec:
      containers:
      - name: xtunnel
        image: ghcr.io/zhboner/xtunnel:latest
        args:
          - "-l=0.0.0.0:9000"
          - "-r=192.168.233.2:9000"
        ports:
        - containerPort: 9000
        resources:
          requests:
            memory: "64Mi"
            cpu: "250m"
          limits:
            memory: "128Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: xtunnel-lb
  namespace: default
spec:
  type: LoadBalancer
  selector:
    app: xtunnel
  ports:
    - name: edge
      port: 9000
```
