use anyhow::Result;
use std::{env, fs::copy, path::PathBuf, process::Command};
use tokio::join;

use super::*;

pub async fn start_generating_wasm_lib() -> Result<()> {
    prepare_output_dir(Platform::Wasm).await?;

    let current_path = env::current_dir()?;
    let working_path = current_path.parent().unwrap().join("target-wasm/src");
    let output_dir = current_path.parent().unwrap().join(format!("output/wasm"));

    env::set_current_dir(&working_path)?;

    let _build = Command::new("cargo")
        .args(["build", "--release"])
        .spawn()?
        .wait_with_output();

    let lib_dir = current_path
        .parent()
        .unwrap()
        .join(format!("target/release"));

    let t1 = tokio::spawn(copy_wasm_file(lib_dir.clone(), output_dir.clone(), File::D));
    let t2 = tokio::spawn(copy_wasm_file(lib_dir, output_dir, File::Dylib));
    let _ = join!(t1, t2);

    env::set_current_dir(&current_path)?;

    Ok(())
}

enum File {
    Dylib,
    D,
}

impl File {
    fn sufix(&self) -> &str {
        match self {
            File::Dylib => "dylib",
            File::D => "d",
        }
    }
}

async fn copy_wasm_file(from: PathBuf, to: PathBuf, file_type: File) -> Result<()> {
    copy(
        from.join(format!("{:}.{:}", WASM, file_type.sufix())),
        to.join(format!("{:}.{:}", WASM, file_type.sufix())),
    )?;

    Ok(())
}
