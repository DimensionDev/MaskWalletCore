use std::env;

use anyhow::Result;
use clap::command;
use colored::Colorize;

mod function;

mod task;
use task::TaskBuilder;

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

    // will call env:set_current_dir for different platform
    // iOS => ./target-mobile
    // Wasm => ./target-wasm
    match matches.subcommand() {
        Some(("ios", args)) => match args.subcommand() {
            Some(("-lib", _)) => {
                println!("{:}\n", "Start generating static lib for iOS".green());
                TaskBuilder::static_lib().await?
            }

            Some(("-xcframework", _)) => {
                println!("{:}\n", "Start generating xcframework for iOS".green());
                TaskBuilder::xcframework().await?
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
            TaskBuilder::wasm().await?
        }

        _ => {
            println!("{:}", "Unsupport command".magenta());
        }
    }

    Ok(())
}
