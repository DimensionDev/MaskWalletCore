use anyhow::Result;

use super::*;

impl TaskBuilder {
    pub async fn static_lib() -> Result<()> {
        let cli_path = current_dir_for_cli(&Platform::iOS)?;
        TaskBuilder::new()
            .task(CliTask::PrepareCliDir(Platform::iOS))
            // generate header file at output path
            .task(CliTask::WriteDotHHeader {
                to: cli_path
                    .parent()
                    .unwrap()
                    .join(format!("output/ios/{:}.h", FRAMEWORK)),
                platform: Platform::iOS,
            })
            // build static lib
            .task(CliTask::Command {
                name: "cargo".to_string(),
                args: ["lipo", "--release"]
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
                excute_path: Option::None,
            })
            .task(CliTask::CopyFile {
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
            .task(CliTask::CreateDir {
                path: cli_path.parent().unwrap().join("output/ios/proto/sign"),
                recursive: true,
            })
            .task(CliTask::Command {
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
            .await
    }
}
