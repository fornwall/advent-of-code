# Fuzzing Advent of Code solutions with AFL
After building an instrumted binary with `cargo afl build` a fuzzing run can be started with `cargo afl fuzz -i testcase-dir -o target/fuzz-findings ../../target/debug/advent-of-code-fuzzing-afl`.

See https://rust-fuzz.github.io/book/afl.html for more information.
