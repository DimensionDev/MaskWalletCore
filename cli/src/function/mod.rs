mod helper;
mod static_lib;
mod xcframework;

use helper::{
    dir_copy, finish, generate_protobuf_files, prepare_output_dir, write_header, FRAMEWORK,
    LIB_NAME,
};
pub use static_lib::start_generating_static_lib;
pub use xcframework::start_generating_xcframework;
