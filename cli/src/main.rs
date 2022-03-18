use anyhow::{anyhow, Result};
use clap::{arg, command};
use colored::Colorize;
use std::{
    convert::AsRef,
    env,
    fs::{copy, create_dir, create_dir_all, metadata, read_dir, remove_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};
use tokio::join;

const LIB_NAME: &'static str = "libmask_wallet_core_mobile.a";
const FRAMEWORK: &'static str = "MaskWalletCoreMobile";

#[tokio::main]
async fn main() -> Result<()> {
    let matches = command!()
        .arg(
            arg!(
                -i --ios [TYPE]... "Generate artifacts of iOS platform, currently support static 'lib' and 'xcframework'"
            )
            .takes_value(true)
            .multiple_values(false),
        )
        .get_matches();

    if matches.is_present("ios") {
        match matches.value_of("ios") {
            Some(artifact) => match artifact {
                "lib" => {
                    println!("{:}\n", "Start generating static lib for iOS".green());
                    start_generating_static_lib().await?
                }

                "xcframework" => {
                    println!("{:}\n", "Start generating xcframework for iOS".green());
                    start_generating_xcframework().await?
                }

                _ => println!(
                    "{:}",
                    "Only static lib and xcframework are supported".magenta()
                ),
            },
            _ => println!("{:}\n", "Sepecify an artifact type".magenta()),
        }
    } else {
        println!("{:}", "Currently artifacts for iOS are available".magenta());
    }

    Ok(())
}

async fn start_generating_xcframework() -> Result<()> {
    prepare_output_dir().await?;

    let xc_dir = tokio::spawn(prepre_xcframework_dirs());
    let build_release = tokio::spawn(cargo_build_release());
    let _ = join!(xc_dir, build_release);
    generate_xcframework().await?;
    Ok(())
}

async fn cargo_build_release() -> Result<()> {
    for target in vec![
        "x86_64-apple-ios",
        "aarch64-apple-ios",
        "aarch64-apple-ios-sim",
    ] {
        let _build = Command::new("cargo")
            .arg("build")
            .arg("--target")
            .arg(target)
            .arg("--release")
            .spawn()?
            .wait_with_output();
    }

    Ok(())
}

async fn prepre_xcframework_dirs() -> Result<()> {
    let xcframework_path = env::current_dir()
        .unwrap()
        .as_path()
        .parent()
        .unwrap()
        .join(format!("output/ios/{:}.xcframework", FRAMEWORK));

    if xcframework_path.exists() {
        remove_dir_all(&xcframework_path)?;
    }
    create_dir(&xcframework_path)?;
    create_dir(&xcframework_path.join(format!("common")))?;
    create_dir(&xcframework_path.join(format!("common/{:}.xcframework", FRAMEWORK)))?;
    let module_path = xcframework_path.join(format!("common/{:}.xcframework/Modules", FRAMEWORK));
    create_dir(&module_path)?;

    let module_map_path = env::current_dir()?
        .as_path()
        .parent()
        .unwrap()
        .join("target-mobile/iOS/module.modulemap");
    copy(module_map_path, module_path.join("module.modulemap"))?;

    let header_path = xcframework_path.join(format!("common/{:}.xcframework/Headers", FRAMEWORK));
    create_dir(&header_path)?;
    write_header(header_path.join(format!("{:}.h", FRAMEWORK))).await?;

    Ok(())
}

async fn generate_xcframework() -> Result<()> {
    let xcframework_path = env::current_dir()
        .unwrap()
        .as_path()
        .parent()
        .unwrap()
        .join(format!("output/ios/{:}.xcframework", FRAMEWORK));
    let common_path = xcframework_path.join(format!("common/{:}.xcframework", FRAMEWORK));
    let arm64_path = xcframework_path.join("ios-arm64");
    let arm64_framework_path = arm64_path.join(format!("{:}.framework", FRAMEWORK));
    create_dir(&arm64_path)?;
    create_dir(&arm64_framework_path)?;
    dir_copy(&common_path, &arm64_path).await?;

    let target_path = env::current_dir().unwrap().parent().unwrap().join("target");
    copy(
        &target_path.join(format!("aarch64-apple-ios/release/{:}", LIB_NAME)),
        &arm64_framework_path.join(format!("{:}", FRAMEWORK)),
    )?;

    let x86_arm64_sim_path = xcframework_path.join("ios-arm64_x86_64-simulator");
    let x86_arm64_sim_framework_path = x86_arm64_sim_path.join(format!("{:}.framework", FRAMEWORK));
    create_dir(&x86_arm64_sim_path)?;
    create_dir(&x86_arm64_sim_framework_path)?;

    dir_copy(&common_path, &x86_arm64_sim_path).await?;

    let _lipo_cmd = Command::new("lipo")
        .arg("-create")
        .arg("-output")
        .arg(xcframework_path.join(format!(
            "ios-arm64_x86_64-simulator/{:}.framework/{:}",
            FRAMEWORK, FRAMEWORK
        )))
        .arg(target_path.join(format!("aarch64-apple-ios-sim/release/{:}", LIB_NAME)))
        .arg(target_path.join(format!("x86_64-apple-ios/release/{:}", LIB_NAME)))
        .spawn()?
        .wait_with_output();

    copy(
        target_path
            .parent()
            .unwrap()
            .join("target-mobile/iOS/Info.plist"),
        xcframework_path.join("Info.plist"),
    )?;
    remove_dir_all(xcframework_path.join("common"))?;

    Ok(())
}

async fn prepare_output_dir() -> Result<()> {
    // mk dir
    let output = env::current_dir()
        .unwrap()
        .as_path()
        .parent()
        .unwrap()
        .join("output");

    // clean output
    if output.exists() {
        remove_dir_all(&output)?;
    }

    create_dir(&output)?;
    create_dir(&output.join("ios"))?;

    Ok(())
}

async fn start_generating_static_lib() -> Result<()> {
    prepare_output_dir().await?;

    let output_header_path = env::current_dir()?
        .as_path()
        .parent()
        .unwrap()
        .join(format!("output/ios/{:}.h", FRAMEWORK));

    let write_header = tokio::spawn(write_header(output_header_path));
    // let generate_protobuf = tokio::spawn(generate_protobuf_files(output));
    let generate_lib = tokio::spawn(generate_static_lib());
    let _ = join!(generate_lib, write_header);

    finish();
    Ok(())
}

fn finish() {
    println!("{:}\n", "cli ==> Done".green());
}

async fn write_header(target: PathBuf) -> Result<()> {
    let cli_path = env::current_dir()?
        .as_path()
        .parent()
        .unwrap()
        .join("target-mobile");
    let target_mobile_lib_path = cli_path.join("src/lib.rs");
    let header_file_path = cli_path.join(format!("{:}.h", FRAMEWORK));
    let mut header_file = File::create(&header_file_path)?;
    let cbindgen_cmd = Command::new("cbindgen")
        .arg(target_mobile_lib_path.to_owned())
        .arg("--crate")
        .arg("target-mobile")
        .arg("-l")
        .arg("c")
        .output()
        .map_err(|_| anyhow!("cbindgen failed"));

    header_file.write_all(&cbindgen_cmd.unwrap().stdout)?;

    copy(header_file_path, target)?;

    Ok(())
}

async fn generate_static_lib() -> Result<()> {
    let _lipo_release = Command::new("cargo")
        .arg("lipo")
        .arg("--release")
        .spawn()?
        .wait_with_output()
        .map_err(|_| anyhow!("failed to generate static lib"));

    let target_lib_path = env::current_dir()?
        .as_path()
        .parent()
        .unwrap()
        .join("target/universal/release/".to_string() + LIB_NAME);
    let lib_path = env::current_dir()?
        .as_path()
        .parent()
        .unwrap()
        .join(format!("output/ios/{:}", LIB_NAME));
    copy(target_lib_path, lib_path)?;

    Ok(())
}

async fn dir_copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<()> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        // println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if metadata(&dest).is_err() {
            // println!(" mkdir: {:?}", dest);
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
                        // println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}

// TODO: protobuf generate
#[allow(dead_code)]
async fn generate_protobuf_files(output: PathBuf) -> Result<()> {
    async fn generate_protos(target: PathBuf, source: PathBuf, err_path: &str) -> Result<()> {
        // let paths = read_dir(&source).unwrap();
        // for path in paths {
        //     let name = path.unwrap().file_name();
        //     let file_path = target.join(name.to_owned());
        //     let protos_cmd = Command::new("protoc")
        //         .arg("--swift_opt=Visibility=Public")
        //         .arg("--swift_out=".to_string() + target.to_str().unwrap())
        //         .arg("--proto_path=".to_string() + file_path.to_str().unwrap())
        //         .output()
        //         .map_err(|_| anyhow!("failed to generato proto in ".to_string() + err_path));
        //     let mut header_file = File::create(file_path)?;
        //     header_file.write_all(&protos_cmd.unwrap().stdout)?;
        // }

        let _protos_cmd = Command::new("protoc")
            .arg("--help")
            // .arg("--proto_path=".to_string() + source.to_str().unwrap())
            // .arg("--swift_opt=Visibility=Public")
            // .arg("--swift_out=".to_string() + target.to_str().unwrap())
            // // .arg(source.to_str().unwrap().to_string() + "/*.proto")
            // .arg("/*.proto")
            .spawn()
            .map_err(|_| anyhow!("failed to generato proto in ".to_string() + err_path));
        println!(
            "{:}\n{:}\n",
            "--swift_out=".to_string() + target.to_str().unwrap(),
            "--proto_path=".to_string() + source.to_str().unwrap()
        );

        Ok(())
    }

    let generate_proto_path = output.join("ios/Protos");
    let protos_path = env::current_dir()
        .unwrap()
        .as_path()
        .parent()
        .unwrap()
        .join("chain-common/proto");

    let generate_proto_sign_path = output.join("ios/Protos/sign");
    let sign_proto_path = env::current_dir()
        .unwrap()
        .as_path()
        .parent()
        .unwrap()
        .join("chain-common/proto/sign");

    create_dir(&generate_proto_path)?;
    create_dir(&generate_proto_sign_path)?;

    let t1 = tokio::spawn(generate_protos(
        generate_proto_path,
        protos_path,
        "chain-common/proto",
    ));
    let t2 = tokio::spawn(generate_protos(
        generate_proto_sign_path,
        sign_proto_path,
        "chain-common/proto/sign",
    ));
    let (_, _) = join!(t1, t2);

    Ok(())
}
