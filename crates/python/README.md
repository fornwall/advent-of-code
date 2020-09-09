# advent-of-code-rs-python
Solutions to [Advent of Code 2019](https://adventofcode.com/2019).

The solutions are implemented in Rust which is compiled to WebAssembly.

# Usage
Add dependency:

```sh
pip install advent-of-code-rs-python
```

The `advent_of_code_rs_python` package exports a single `solve` function with the following signature:

```js
solve(day: int, part: int, input: str): str
```

Examples:

```python
from advent_of_code_rs_python import solve

assert "3" == solve(1, 1, "14")
assert "30" == solve(3, 2, "R8,U5,L5,D3\nU7,R6,D4,L4")
```
