use anyhow::{anyhow, Result};
use colored::Colorize;
use std::{
    convert::AsRef,
    env,
    fs::{copy, create_dir, create_dir_all, metadata, read_dir, remove_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

pub(crate) const LIB_NAME: &'static str = "libmask_wallet_core_mobile";
pub(crate) const FRAMEWORK: &'static str = "MaskWalletCoreMobile";
pub(crate) const WASM: &'static str = "libmask_wallet_core_wasm";

#[non_exhaustive]
pub enum Platform {
    #[allow(non_camel_case_types)]
    iOS,
    Wasm,
}

pub async fn prepare_output_dir(platform: Platform) -> Result<()> {
    fn check_and_create_dir(path: PathBuf) -> Result<()> {
        if path.exists() {
            remove_dir_all(&path)?;
        }
        create_dir(&path)?;

        Ok(())
    }

    // mk dir
    let output = env::current_dir()?.parent().unwrap().join("output");

    // clean output
    if !output.exists() {
        create_dir(&output)?;
    }

    let path = match platform {
        Platform::iOS => output.join("ios"),
        Platform::Wasm => output.join("wasm"),
    };
    check_and_create_dir(path)?;

    Ok(())
}

pub fn finish() {
    println!("{:}\n", "cli ==> Done".green());
}

pub async fn write_header(target: PathBuf) -> Result<()> {
    let cli_path = env::current_dir()?.parent().unwrap().join("target-mobile");
    let target_mobile_lib_path = cli_path.join("src/lib.rs");
    let header_file_path = cli_path.join(format!("{:}.h", FRAMEWORK));
    let mut header_file = File::create(&header_file_path)?;
    let cbindgen_cmd = Command::new("cbindgen")
        .args([
            target_mobile_lib_path.to_str().unwrap(),
            "--crate",
            "target-mobile",
            "-l",
            "c",
        ])
        .output()
        .map_err(|_| anyhow!("cbindgen failed"));

    header_file.write_all(&cbindgen_cmd.unwrap().stdout)?;

    copy(header_file_path, target)?;

    Ok(())
}

pub async fn dir_copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<()> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };

        if metadata(&dest).is_err() {
            create_dir_all(&dest)?;
        }

        for entry in read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        // println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed to push path: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}

//
/// protobuf generation didn't work via this Command, here we use a shell script as a workaround
/// let _protos_cmd = Command::new("protoc")
/// .arg("--swift_opt=Visibility=Public")
/// .arg("--swift_out=/ ".to_string() + &slice)
/// .arg("-I=./")
/// .spawn()
/// .map_err(|_| anyhow!("failed to generato proto in ".to_string() + err_path));
pub async fn generate_protobuf_files(output: PathBuf) -> Result<()> {
    let current_dir = env::current_dir()?;
    let generate_proto_path = output.join("ios/proto");
    let protos_path = current_dir.parent().unwrap().join("chain-common/proto");
    let generate_proto_sign_path = output.join("ios/proto/sign");
    let script_path = current_dir.parent().unwrap().join("scripts/proto.sh");

    create_dir(&generate_proto_path)?;
    create_dir(&generate_proto_sign_path)?;

    env::set_current_dir(&protos_path)?;
    let _sh = Command::new("sh")
        .arg(script_path.to_owned())
        .arg("--push")
        .arg("false")
        .spawn()?
        .wait_with_output();
    env::set_current_dir(&current_dir)?;

    Ok(())
}
