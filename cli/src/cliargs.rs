use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {
    #[command(subcommand)]
    pub operation: CliOperation
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum CliOperation {

    Set {
        key: String,
        value: String
    },

    Get {
        key: String
    }
}