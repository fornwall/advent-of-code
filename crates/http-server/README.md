[![Docker Hub](https://img.shields.io/docker/v/fredrikfornwall/advent-of-code-http-server.svg?label=docker)](https://hub.docker.com/r/fredrikfornwall/advent-of-code-http-server)

# Advent of Code HTTP API server

A HTTP server exposing an API to solve [Advent of Code](https://adventofcode.com/) problems.

Solutions are implemented in Rust in the [core crate](https://github.com/fornwall/advent-of-code/tree/master/crates/core) and this crate uses the [warp](https://github.com/seanmonstar/warp/) library to expose them over a HTTP API.

- Deployment URL: `https://advent.fly.dev`
- OpenAPI schema: https://aoc.fornwall.net/openapi.json
- Swagger UI: https://aoc.fornwall.net/api/

The HTTP API expects a `POST` to `/solve/$YEAR/$DAY/$PART` with the problem input as post body text. It can be invoked using [curl](https://curl.se/) as shown below:

```sh
curl -d 14 https://advent.fly.dev/solve/2019/1/1
```

A [fredrikfornwall/advent-of-code-http-server](https://hub.docker.com/r/fredrikfornwall/advent-of-code-http-server) Docker image which starts the server on port 8080 is also available on Docker Hub:

```sh
docker run -p 8080:8080 fredrikfornwall/advent-of-code-http-server
```
