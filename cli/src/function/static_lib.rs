use anyhow::{anyhow, Result};
use std::{fs::copy, path::PathBuf, process::Command};
use tokio::join;

use super::*;

pub async fn start_generating_static_lib() -> Result<()> {
    prepare_output_dir(Platform::iOS).await?;

    let output = current_dir()?.parent().unwrap().join(format!("output"));
    let output_header_path = output.join(format!("ios/{:}.h", FRAMEWORK));

    let write_header = tokio::spawn(write_header(output_header_path, &Platform::iOS));
    let generate_lib = tokio::spawn(generate_static_lib());
    let _ = join!(generate_lib, write_header);
    let _ = generate_protobuf_files(output).await;

    finish();
    Ok(())
}

fn current_dir() -> Result<PathBuf> {
    current_dir_for_cli(&Platform::iOS)
}

async fn generate_static_lib() -> Result<()> {
    let _lipo_release = Command::new("cargo")
        .arg("lipo")
        .arg("--release")
        .spawn()?
        .wait_with_output()
        .map_err(|_| anyhow!("failed to generate static lib"));

    let target_lib_path = current_dir()?
        .parent()
        .unwrap()
        .join(format!("target/universal/release/{:}.a", LIB_NAME));
    let lib_path = current_dir()?
        .parent()
        .unwrap()
        .join(format!("output/ios/{:}.a", LIB_NAME));
    copy(target_lib_path, lib_path)?;

    Ok(())
}
