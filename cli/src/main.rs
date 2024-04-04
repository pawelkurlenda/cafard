mod cliargs;

use crate::cliargs::{CliArgs, CliOperation};
use clap::Parser;

fn main() {
    let cli_args = CliArgs::parse();

    match cli_args.operation {
        CliOperation::SET { key, value} => {
            println!("Set Key: {}, Value: {}", key, value)
        },
        CliOperation::GET { key} => {
            println!("Get value Key: {}", key)
        },
        CliOperation::DELETE { key} => {
            println!("Delete value Key: {}", key)
        }
    }
}
