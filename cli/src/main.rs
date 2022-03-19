use anyhow::Result;
use clap::{arg, command};
use colored::Colorize;
use std::env;

mod function;
use function::start_generating_static_lib;
use function::start_generating_xcframework;

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