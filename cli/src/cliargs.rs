use clap::{Parser, Subcommand};

/// Main CLI Application
///
/// This CLI tool allows for basic operations on a key-value store, including setting,
/// getting, and deleting values associated with keys.
#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "1.0.0", about = "A simple key-value store CLI", long_about = None)]
pub(crate) struct CliArgs {
    /// The operation to perform
    #[command(subcommand)]
    pub operation: CliOperation
}

/// Supported Operations
///
/// SET: Assigns a value to a specified key.
/// GET: Retrieves the value for a specified key.
/// DELETE: Removes a specified key and its associated value.
#[derive(Subcommand, Debug, Clone)]
pub(crate) enum CliOperation {
    /// Sets a key to a specified value
    ///
    /// This operation will store the value provided under the specified key.
    /// If the key already exists, its value will be overwritten.
    SET {
        /// The unique identifier for the value
        #[arg(help = "The key to set")]
        key: String,

        /// The value to associate with the key
        #[arg(help = "The value to be set for the key")]
        value: String,
    },

    /// Retrieves the value associated with a key
    ///
    /// This operation will return the value stored under the specified key, if it exists.
    GET {
        /// The unique identifier whose value you want to retrieve
        #[arg(help = "The key to get the value for")]
        key: String,
    },

    /// Deletes a key and its associated value
    ///
    /// This operation will remove the specified key and its value from the store,
    /// effectively deleting it.
    DELETE {
        /// The unique identifier whose associated value you want to delete
        #[arg(help = "The key to delete")]
        key: String,
    },
}