use std::env;
extern crate prost_build;

fn main() {
    env::set_var("OUT_DIR", "src");
    prost_build::compile_protos(
        &[
            "proto/Api.proto",
            "proto/Param.proto",
        ],
        &["proto/"],
    )
    .unwrap();


}