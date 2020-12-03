[![PyPi](https://img.shields.io/pypi/v/advent-of-code.svg)](https://pypi.org/project/advent-of-code/)

# advent-of-code-python
Solutions to [Advent of Code](https://adventofcode.com/) implemented in Rust and exposed to Python using [PyO3](https://pyo3.rs/).

# Usage as a library
Add dependency:

```sh
pip install --upgrade advent-of-code
```

The `advent_of_code` package exports a single `solve` function with the following signature:

```js
def solve(day: int, part: int, input: str) -> str
```

Examples:

```python
from advent_of_code import solve

assert "3" == solve(1, 1, "14")
assert "30" == solve(3, 2, "R8,U5,L5,D3\nU7,R6,D4,L4")
```

# Usage as a command line tool

```sh
$ pip install --upgrade advent-of-code
$ echo 14 | advent-of-code-py 2019 1 1
2
```
