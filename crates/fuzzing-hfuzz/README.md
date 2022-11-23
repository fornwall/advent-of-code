# Fuzzing Advent of Code solutions with Honggfuzz
See [hongfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs).

After installing honggfuzz, run fuzzing with `cargo hfuzz run advent-of-code-fuzzing-hfuzz`.

If a crash is found, inputs causing that will be saved as files under `hfuzz_workspace/advent-of-code-fuzzing-hfuzz/`, using the `.fuzz` file extension.
