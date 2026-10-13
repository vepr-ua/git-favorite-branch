// use crate::git_helpers;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add current branch to favorites
    Add {
        key: String,
        branch_name: Option<String>,
    },
    /// Switch to a different branch
    Use { key: String },
    /// Remove favorite branch
    Del { key: String },
    /// Delete all favorite branches
    DelAll,
    /// Print Branch Name
    Branch { key: Option<String> },
    /// Create a new branch that is named with a value for a given key
    New { key: String },
    /// List all favorite branches
    Ls,
    /// Rename a saved key
    ///
    /// `gfb rename <NEW_KEY>` renames the key saved for the branch you are on.
    /// That branch must be saved under exactly one key.
    ///
    /// `gfb rename <OLD_KEY> <NEW_KEY>` renames a key by name, from anywhere.
    ///
    /// The new key must not already exist.
    #[command(override_usage = "gfb rename <NEW_KEY>\n       gfb rename <OLD_KEY> <NEW_KEY>")]
    Rename {
        #[arg(required = true, num_args = 1..=2, value_name = "KEY")]
        keys: Vec<String>,
    },
}
