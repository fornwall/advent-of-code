# Advent of Code solver gRPC server
Start server with `cargo run`, invoke as with:

```sh
grpcurl -plaintext \
  -proto ./proto/advent.proto \
  -d '{"year": 2019, "day": 1, "part": 1, "text": "12334"}' \
  '[::]:50051' \
  advent.Solver/Solve
```

Show a UI with:

```sh
grpcui -proto proto/advent.proto -plaintext '[::]:50051'
```
