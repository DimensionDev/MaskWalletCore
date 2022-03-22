use cbindgen::{Builder, Language};
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    Builder::new()
        .with_language(Language::C)
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("MaskWalletCoreMobile.h");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}