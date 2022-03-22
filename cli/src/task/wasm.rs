use anyhow::Result;

use super::*;

// wasm
impl TaskBuilder {
    pub async fn wasm() -> Result<()> {
        let cli_path = current_dir_for_cli(&Platform::iOS)?;

        TaskBuilder::new()
            .task(Task::PrepareCliDir(Platform::Wasm))
            .task(Task::Command {
                name: "cargo".to_owned(),
                args: ["build", "--release"]
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
                excute_path: build_command_excute_path(&Platform::Wasm)?.into(),
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
            .await
    }
}
