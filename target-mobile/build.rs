use cbindgen::Language;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let bindings = cbindgen::Builder::new()
        .with_language(Language::C)
        .with_crate(crate_dir)
        .generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file("iOS/MaskWalletCoreMobile.h");
    bindings.write_to_file("iOS/libmask_wallet_core_mobile.h");
}
