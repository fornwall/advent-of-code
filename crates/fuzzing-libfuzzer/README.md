# Fuzzing Advent of Code solutions with libFuzzer
See [libfuzzer-sys](https://github.com/rust-fuzz/libfuzzer).

It uses [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz), which requires a special directory structure.

This directory (the `Cargo.toml` and `src/lib.rs`) are just empty placeholders to make `cargo-fuzz` happy.

The real folder of interest is [fuzz/](fuzz/), especially [Cargo.toml](fuzz/Cargo.toml) and [fuzz_target.rs](fuzz/fuzz_targets/fuzz_target.rs) there.

Run the fuzzer with something like `cargo +nightly fuzz run fuzz_target -- -max_total_time=1800`.

# Reference
See [Rust Fuzz Book - Fuzzing with cargo-fuzz](https://rust-fuzz.github.io/book/cargo-fuzz.html).
