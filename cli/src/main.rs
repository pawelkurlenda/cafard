mod cliargs;

use crate::cliargs::CliArgs;
use clap::Parser;

fn main() {
    let cli_args = CliArgs::parse();
}
