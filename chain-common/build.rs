use prost_build::Config;

fn main() {
    Config::new()
        .out_dir("src/generated")
        .compile_protos(&["proto/api.proto"], &["proto/sign/", "proto/"])
        .expect("failed to generate protos");
}
