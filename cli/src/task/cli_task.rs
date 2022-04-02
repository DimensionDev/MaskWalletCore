use std::path::PathBuf;

use super::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
#[non_exhaustive]
pub enum CliTask {
    PrepareCliDir(Platform),
    CreateDir {
        path: PathBuf,
        recursive: bool,
    },
    RemoveDirAll(PathBuf),
    CopyDir {
        from: PathBuf,
        to: PathBuf,
    },
    CopyFile {
        from: PathBuf,
        to: PathBuf,
    },
    Command {
        name: String,
        args: Vec<String>,
        excute_path: Option<PathBuf>,
    },
    WriteDotHHeader {
        to: PathBuf,
        platform: Platform,
    },
}
