use pb_rs::{types::FileDescriptor, ConfigBuilder};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Directories for generated files
    let proto_dir = PathBuf::from("src/protos");

    let prost_out_dir = proto_dir.join("prost");
    let protobuf_out_dir = proto_dir.join("protobuf");
    let quick_out_dir = proto_dir.join("quick");

    // Create the output directories if they don't exist
    fs::create_dir_all(&prost_out_dir).unwrap();
    fs::create_dir_all(&protobuf_out_dir).unwrap();
    fs::create_dir_all(&quick_out_dir).unwrap();

    // Compile protobuf files using protobuf-codegen (rust-protobuf)
    eprintln!("Running protobuf-codegen...");
    protobuf_codegen::Codegen::new()
        .out_dir(protobuf_out_dir.to_str().unwrap()) // Output directory for generated files
        .inputs(["src/protos/benchmark.proto"])
        .include("src/protos")
        .run()
        .expect("protobuf-codegen failed");
    eprintln!("protobuf-codegen completed successfully");

    // Compile protobuf files using prost-build
    eprintln!("Running prost-build...");
    prost_build::Config::new()
        .out_dir(prost_out_dir.to_str().unwrap()) // Output directory for generated files
        .compile_protos(&["src/protos/benchmark.proto"], &["src/protos"])
        .expect("prost-build failed");
    eprintln!("prost-build completed successfully");

    // Rename the generated `_.rs` file to `benchmark.rs`
    let generated_file_path = prost_out_dir.join("_.rs");
    let renamed_file_path = prost_out_dir.join("benchmark.rs");
    if generated_file_path.exists() {
        fs::rename(&generated_file_path, renamed_file_path)
            .expect("Failed to rename prost generated file");
    }

    // Create a mod.rs for prost
    let mut prost_mod_file = File::create(prost_out_dir.join("mod.rs")).unwrap();
    writeln!(prost_mod_file, "pub mod benchmark;").unwrap();

    // Inform Cargo to re-run this script if the proto file changes
    println!("cargo:rerun-if-changed=src/protos/benchmark.proto");

    let config_builder = ConfigBuilder::new(
        &["src/protos/benchmark.proto"],
        None,
        Some(&quick_out_dir.to_str().unwrap()),
        &["src/protos"],
    )
    .unwrap();
    FileDescriptor::run(&config_builder.build()).unwrap()
}
