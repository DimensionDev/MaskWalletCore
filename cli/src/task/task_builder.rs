use std::{
    env,
    fs::{copy, create_dir, create_dir_all, remove_dir_all},
    process::Command,
};

use anyhow::Result;
use tokio::task::JoinHandle;

use super::*;

pub struct TaskBuilder {
    pub tasks: Vec<Task>,
}

impl TaskBuilder {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn task(mut self, task: Task) -> Self {
        self.tasks.push(task);
        self
    }
}

impl TaskBuilder {
    pub async fn run(&self) -> Result<()> {
        fn spawn_handle_for(task: Task) -> JoinHandle<Result<()>> {
            match task {
                Task::PrepareCliDir(platform) => tokio::spawn(prepare_output_dir(platform)),

                Task::CopyDir { from, to } => tokio::spawn(async {
                    if !to.exists() {
                        create_dir_all(to.clone())?;
                    }

                    dir_copy(from, to).await?;

                    Ok(())
                }),

                Task::CreateDir { path, recursive } => tokio::spawn(async move {
                    if recursive {
                        create_dir_all(path)?;
                    } else {
                        create_dir(path)?;
                    }

                    Ok(())
                }),

                Task::RemoveDirAll(path) => tokio::spawn(async move {
                    remove_dir_all(path)?;
                    Ok(())
                }),

                Task::CopyFile { from, to } => tokio::spawn(async {
                    copy(from, to)?;
                    Ok(())
                }),

                Task::Command {
                    name,
                    args,
                    excute_path,
                } => tokio::spawn(async move {
                    let mut dir_changed = false;
                    let current_path = env::current_dir()?;
                    if let Some(path) = excute_path {
                        env::set_current_dir(path)?;
                        dir_changed = true;
                    }

                    let _ad = Command::new(&name)
                        .args(args)
                        .spawn()?
                        .wait_with_output()
                        .map_err(|_| anyhow::anyhow!("failed when excuting {:}", name));

                    if dir_changed {
                        env::set_current_dir(current_path)?;
                    }

                    Ok(())
                }),

                Task::WriteDotHHeader { to, platform } => tokio::spawn(write_header(to, platform)),
            }
        }

        for task in self.tasks.clone() {
            let _ = spawn_handle_for(task).await;
        }

        finish();

        Ok(())
    }
}
