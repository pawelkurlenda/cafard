use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub(crate) struct CliArgs {
    #[arg(required = true)]
    pub operation: CliOperation
}

#[derive(Subcommand, Debug)]
pub enum CliOperation {
    Set {
        key: String,
        value: String
    },
    Get {
        key: String
    }
}