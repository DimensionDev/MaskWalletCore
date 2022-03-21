mod helper;
mod static_lib;
mod wasm;
mod xcframework;

use helper::{
    current_dir_for_cli, dir_copy, finish, generate_protobuf_files, prepare_output_dir,
    write_header, Platform, FRAMEWORK, LIB_NAME, WASM,
};

pub use static_lib::start_generating_static_lib;
pub use wasm::start_generating_wasm_lib;
pub use xcframework::start_generating_xcframework;
