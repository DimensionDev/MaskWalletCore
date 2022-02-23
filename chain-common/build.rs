use prost_build;

fn main() {
    prost_build::Config::new()
        .out_dir("src/generated")
        .compile_protos(&["proto/api.proto"], &["proto/sign/", "proto/"])
        .expect("failed to generate protos");
}
