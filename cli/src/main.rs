use std::env;

use anyhow::Result;
use clap::command;
use colored::Colorize;

mod function;
use function::start_generating_static_lib;
use function::start_generating_wasm_lib;
use function::start_generating_xcframework;

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
                start_generating_static_lib().await?
            }

            Some(("-xcframework", _)) => {
                println!("{:}\n", "Start generating xcframework for iOS".green());
                start_generating_xcframework().await?
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
            start_generating_wasm_lib().await?
        }

        _ => {
            println!("{:}", "Unsupport command".magenta());
        }
    }

    Ok(())
}
