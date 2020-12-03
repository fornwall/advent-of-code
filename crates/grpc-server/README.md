[![Docker Hub](https://img.shields.io/docker/v/fredrikfornwall/advent-of-code-grpc-server.svg?label=docker)](https://hub.docker.com/r/fredrikfornwall/advent-of-code-grpc-server)

# Advent of Code gRPC solver server
A gRPC server exposing an API to solve [Advent of Code](https://adventofcode.com/) problems.

Solutions are implemented in Rust in the [core crate](https://github.com/fornwall/advent-of-code/tree/master/crates/core) and this crate uses the [Tonic](https://docs.rs/tonic/0.3.1/tonic/) library to expose them over gRPC.

- Deployment URL: `advent-grpc.fly.dev:443`
- Schema: [proto/advent.proto](proto/advent.proto)

It can be invoked using [grpcurl](https://github.com/fullstorydev/grpcurl) as shown below:

```sh
grpcurl -plaintext \
  -proto ./proto/advent.proto \
  -d '{"year": 2019, "day": 1, "part": 1, "text": "12334"}' \
  advent-grpc.fly.dev:443 \
  advent.Solver/Solve
```

Show a UI using [grpcui](https://github.com/fullstorydev/grpcui):

```sh
grpcui -proto proto/advent.proto -plaintext advent-grpc.fly.dev:443
```

A [fredrikfornwall/advent-of-code-grpc-server](https://hub.docker.com/r/fredrikfornwall/advent-of-code-grpc-server) Docker image which starts the server on port 50051 is also available on Docker Hub:

```sh
docker run -p 50051:50051 fredrikfornwall/advent-of-code-grpc-server
```
