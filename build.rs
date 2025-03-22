use std::path::PathBuf;
use std::{env, fs};

fn main() {
    protobuf_codegen::Codegen::new()
        .cargo_out_dir("protos")
        .include("api")
        .input("api/bloop.proto")
        .run_from_script();

    let target_directory = PathBuf::from("target");
    fs::create_dir_all(&target_directory).expect("Unable to create target directory");

    let header_file = target_directory.join("include").join("bloop").join("bloop.h");

    let crate_directory = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::generate(crate_directory)
        .expect("Unable to generate bindings")
        .write_to_file(header_file);
}
