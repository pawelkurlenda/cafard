use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {
    #[command(subcommand)]
    pub operation: CliOperation
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum CliOperation {
    #[arg(short = "S", long = "SET")]
    SET {
        key: String,
        value: String
    },

    #[arg(short = "G", long = "GET")]
    GET {
        key: String
    },

    #[arg(short = "D", long = "DELETE")]
    DELETE {
        key: String
    }
}