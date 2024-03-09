use clap::{Parser};

#[derive(Parser, Debug)]
pub(crate) struct CliArgs {
    #[arg(required = true)]
    text: String
}