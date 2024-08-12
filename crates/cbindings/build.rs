extern crate cbindgen;

use std::env;
use std::path::PathBuf;

use cbindgen::Config;

fn main() {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let generated_header_path = target_dir().join("advent-of-code.h");

    let config = Config {
        cpp_compat: true,
        ..Default::default()
    };

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .with_include_guard("ADVENT_OF_CODE_H")
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(generated_header_path);
}

/// Find the location of the `target/` directory. Note that this may be
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
/// variable.
fn target_dir() -> PathBuf {
    #![allow(clippy::unwrap_used)]
    env::var("CARGO_TARGET_DIR").map_or_else(
        |_| PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target"),
        PathBuf::from,
    )
}
