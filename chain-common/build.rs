use std::env;
extern crate prost_build;

fn main() {
    env::set_var("OUT_DIR", "src/generated");
    prost_build::compile_protos(
        &[
            "proto/api.proto",
        ],
        &[
            "proto/sign/",
            "proto/",
        ],
    )
    .unwrap();
}