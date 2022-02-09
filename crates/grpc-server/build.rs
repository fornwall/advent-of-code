use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("proto");
    let proto_file = root.join("advent.proto");

    // Tell cargo to recompile if proto file is changed:
    println!("cargo:rerun-if-changed={}", proto_file.display());

    // "Generate a file containing the encoded prost_types::FileDescriptorSet for protocol
    // buffers modules. This is required for implementing gRPC Server Reflection":
    let descriptor_path = PathBuf::from(env::var("OUT_DIR")?).join("advent.bin");

    tonic_build::configure()
        .file_descriptor_set_path(&descriptor_path)
        .compile(&[proto_file.as_path()], &[root.as_path()])?;

    Ok(())
}
