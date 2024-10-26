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

