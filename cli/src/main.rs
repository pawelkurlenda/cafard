mod cliargs;

use crate::cliargs::{CliArgs, CliOperation};
use clap::Parser;

fn main() {
    let cli_args = CliArgs::parse();

    match cli_args.operation {
        CliOperation::Set { key, value} => {
            println!("Set Key: {}, Value: {}", key, value)
        }
        CliOperation::Get { key} => {
            println!("Get value Key: {}", key)
        }
    }
}
