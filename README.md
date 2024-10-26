# Adaptive Limiting

Adaptive rate limiting example project that shows how a web service and reverse proxy (HA Proxy) behaves as the latency and error rates of the web service changes dynamically.

The project contains 3 components

1. A rust web service with a json API that has 3 endpoints
   1. GET returns a 'hello' response after the configured timeout or 500 error at a defined error rate
   2. POST for the latency of the 'hello' response
   3. POST for the error rate (a % of requests that should be faild 0-100)
4. A rust load application that makes calls to the hello service and reports to the console the number of reponses per second and the number of errors
5. A rust config application that updates the latency or error rate for the hello service.

The proxy and web service are composed into a k8s application. The proxy endpoint is forwarded to the host for local access.

## Kubernetes Deployment

To deploy the Kubernetes app with HAProxy configured to manage rate limits using adaptive rate limits, follow these steps:

1. Ensure you have `kubectl` and a Kubernetes cluster set up.
2. Apply the Kubernetes deployment and service configurations:

```sh
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml
```

## HAProxy Configuration

The HAProxy configuration file (`haproxy/haproxy.cfg`) is used to manage rate limits using adaptive rate limits. The configuration includes:

- Listening on port 80
- Defining a backend for the web service
- Setting up rate limiting using stick tables
- Configuring adaptive rate limits based on response times
- Forwarding requests to the web service backend

## Configuration Files

The following configuration files have been added to the repository:

- `k8s/deployment.yaml`: Defines the deployments for the web service and HAProxy.
- `k8s/service.yaml`: Defines the services for the web service and HAProxy.
- `haproxy/haproxy.cfg`: Contains the HAProxy configuration for managing rate limits.
