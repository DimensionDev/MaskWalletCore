use std::env;

use anyhow::Result;
use clap::command;
use colored::Colorize;

mod function;
use function::{current_dir_for_cli, Platform, FRAMEWORK, LIB_NAME, WASM};

mod task;
use task::{Task, TaskBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = command!()
        .subcommand(
            command!("ios")
                .subcommand(command!("-lib").about("Generate static lib of iOS platform"))
                .subcommand(
                    command!("-xcframework").about("Generate xcframework for iOS platform"),
                ),
        )
        .subcommand(command!("wasm").about("Generate wasm dylib"))
        .get_matches();

    // will call end:set_current_dir for different platform
    // iOS => ./target-mobile
    // Wasm => ./target-wasm
    match matches.subcommand() {
        Some(("ios", args)) => match args.subcommand() {
            Some(("-lib", _)) => {
                println!("{:}\n", "Start generating static lib for iOS".green());

                let cli_path = current_dir_for_cli(&Platform::iOS)?;
                TaskBuilder::new()
                    .task(Task::PrepareCliDir(Platform::iOS))
                    // generate header file at output path
                    .task(Task::WriteDotHHeader {
                        to: cli_path
                            .parent()
                            .unwrap()
                            .join(format!("output/ios/{:}.h", FRAMEWORK)),
                        platform: Platform::iOS,
                    })
                    // build static lib
                    .task(Task::Command {
                        name: "cargo".to_string(),
                        args: ["lipo", "--release"]
                            .into_iter()
                            .map(|x| x.to_string())
                            .collect(),
                        excute_path: Option::None,
                    })
                    .task(Task::CopyFile {
                        from: cli_path
                            .parent()
                            .unwrap()
                            .join(format!("target/universal/release/{:}.a", LIB_NAME)),
                        to: cli_path
                            .parent()
                            .unwrap()
                            .join(format!("output/ios/{:}.a", LIB_NAME)),
                    })
                    // protobuf files
                    .task(Task::CreateDir {
                        path: cli_path.parent().unwrap().join("output/ios/proto/sign"),
                        recursive: true,
                    })
                    .task(Task::Command {
                        name: "sh".to_string(),
                        args: [
                            cli_path
                                .parent()
                                .unwrap()
                                .join("scripts/proto.sh")
                                .to_str()
                                .unwrap(),
                            "--push",
                            "false",
                        ]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect(),
                        excute_path: Some(cli_path.parent().unwrap().join("chain-common/proto")),
                    })
                    .run()
                    .await?
            }

            Some(("-xcframework", _)) => {
                println!("{:}\n", "Start generating xcframework for iOS".green());
                // start_generating_xcframework().await?;
                let cli_path = current_dir_for_cli(&Platform::iOS)?;
                let xcframework_path = cli_path
                    .parent()
                    .unwrap()
                    .join(format!("output/ios/{:}.xcframework", FRAMEWORK));
                TaskBuilder::new()
                    .task(Task::PrepareCliDir(Platform::iOS))
                    // modulemap
                    // note plz write or copy some file after create the dir before create another dir on the inherit path
                    .task(Task::CreateDir {
                        path: xcframework_path
                            .join(format!("common/{:}.xcframework/Modules", FRAMEWORK)),
                        recursive: true,
                    })
                    .task(Task::CopyFile {
                        from: cli_path
                            .parent()
                            .unwrap()
                            .join("target-mobile/iOS/module.modulemap"),
                        to: xcframework_path.join(format!(
                            "common/{:}.xcframework/Modules/module.modulemap",
                            FRAMEWORK
                        )),
                    })
                    // generate header file at output path
                    .task(Task::CreateDir {
                        path: xcframework_path
                            .join(format!("common/{:}.xcframework/Headers", FRAMEWORK)),
                        recursive: true,
                    })
                    .task(Task::WriteDotHHeader {
                        to: xcframework_path.join(format!(
                            "common/{:}.xcframework/Headers/{:}.h",
                            FRAMEWORK, FRAMEWORK
                        )),
                        platform: Platform::iOS,
                    })
                    // build xcframework
                    .task(Task::Command {
                        name: "cargo".to_string(),
                        args: ["build", "--target", "x86_64-apple-ios", "--release"]
                            .into_iter()
                            .map(|x| x.to_string())
                            .collect(),
                        excute_path: cli_path.clone().into(),
                    })
                    .task(Task::Command {
                        name: "cargo".to_string(),
                        args: ["build", "--target", "aarch64-apple-ios-sim", "--release"]
                            .into_iter()
                            .map(|x| x.to_string())
                            .collect(),
                        excute_path: cli_path.clone().into(),
                    })
                    .task(Task::Command {
                        name: "cargo".to_string(),
                        args: ["build", "--target", "aarch64-apple-ios", "--release"]
                            .into_iter()
                            .map(|x| x.to_string())
                            .collect(),
                        excute_path: cli_path.clone().into(),
                    })
                    // xcframework dir
                    .task(Task::CopyDir {
                        from: xcframework_path
                            .clone()
                            .join(format!("common/{:}.xcframework", FRAMEWORK)),
                        to: xcframework_path
                            .clone()
                            .join(format!("ios-arm64/{:}.framework", FRAMEWORK)),
                    })
                    .task(Task::CopyFile {
                        from: cli_path
                            .parent()
                            .unwrap()
                            .join(format!("target/aarch64-apple-ios/release/{:}.a", LIB_NAME)),
                        to: xcframework_path
                            .join(format!("ios-arm64/{:}.framework/{:}", FRAMEWORK, FRAMEWORK)),
                    })
                    .task(Task::CopyDir {
                        from: xcframework_path
                            .clone()
                            .join(format!("common/{:}.xcframework", FRAMEWORK)),
                        to: xcframework_path.join(format!(
                            "ios-arm64_x86_64-simulator/{:}.framework",
                            FRAMEWORK
                        )),
                    })
                    .task(Task::Command {
                        name: "lipo".to_string(),
                        args: [
                            "-create",
                            "-output",
                            xcframework_path
                                .join(format!(
                                    "ios-arm64_x86_64-simulator/{:}.framework/{:}",
                                    FRAMEWORK, FRAMEWORK
                                ))
                                .to_str()
                                .unwrap(),
                            cli_path
                                .parent()
                                .unwrap()
                                .join("target")
                                .join(format!("aarch64-apple-ios-sim/release/{:}.a", LIB_NAME))
                                .to_str()
                                .unwrap(),
                            cli_path
                                .parent()
                                .unwrap()
                                .join("target")
                                .join(format!("x86_64-apple-ios/release/{:}.a", LIB_NAME))
                                .to_str()
                                .unwrap(),
                        ]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect(),
                        excute_path: Option::None,
                    })
                    .task(Task::CopyFile {
                        from: cli_path
                            .parent()
                            .unwrap()
                            .join("target-mobile/iOS/Info.plist"),
                        to: xcframework_path.join("Info.plist"),
                    })
                    .task(Task::RemoveDirAll(xcframework_path.join("common")))
                    // protobuf files
                    .task(Task::CreateDir {
                        path: cli_path.parent().unwrap().join("output/ios/proto/sign"),
                        recursive: true,
                    })
                    .task(Task::Command {
                        name: "sh".to_string(),
                        args: [
                            cli_path
                                .parent()
                                .unwrap()
                                .join("scripts/proto.sh")
                                .to_str()
                                .unwrap(),
                            "--push",
                            "false",
                        ]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect(),
                        excute_path: cli_path.parent().unwrap().join("chain-common/proto").into(),
                    })
                    .run()
                    .await?
            }

            _ => {
                println!(
                    "{:}",
                    "Only static lib and xcframework are supported".magenta()
                )
            }
        },

        Some(("wasm", _)) => {
            println!("{:}\n", "Start generating wasm lib".green());
            let cli_path = current_dir_for_cli(&Platform::iOS)?;

            TaskBuilder::new()
                .task(Task::PrepareCliDir(Platform::Wasm))
                .task(Task::Command {
                    name: "cargo".to_owned(),
                    args: ["build", "--release"]
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect(),
                    excute_path: cli_path.parent().unwrap().join("target-wasm/src").into(),
                })
                .task(Task::CopyFile {
                    from: cli_path
                        .parent()
                        .unwrap()
                        .join(format!("target/release/{:}.{:}", WASM, "dylib")),
                    to: cli_path
                        .parent()
                        .unwrap()
                        .join(format!("output/wasm/{:}.{:}", WASM, "dylib")),
                })
                .task(Task::CopyFile {
                    from: cli_path
                        .parent()
                        .unwrap()
                        .join(format!("target/release/{:}.{:}", WASM, "D")),
                    to: cli_path
                        .parent()
                        .unwrap()
                        .join(format!("output/wasm/{:}.{:}", WASM, "D")),
                })
                .run()
                .await?
        }

        _ => {
            println!("{:}", "Unsupport command".magenta());
        }
    }

    Ok(())
}
