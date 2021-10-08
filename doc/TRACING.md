# Tracing

## Jaeger

```shell
$ docker pull jaegertracing/all-in-one:1.27.0
$ docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:1.27.0
```

## Zipkin

```shell
$ docker pull openzipkin/zipkin:2.23.2
$ docker run -d -p 9411:9411 openzipkin/zipkin:2.23.2
```
