//! # CLI settings
//!
//! This module contains the cli settings of the program.
//!
//! ## Arguments
//! The first given argument is one of the commands. The second is a file or, in case of create, there is the possibility to pass flags.

use camino::Utf8PathBuf;
use clap::{ArgAction, Args, Parser, Subcommand};

pub mod prompts;

/// CLI interface of Episko
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new manifest file
    Create(CreateArgs),
    /// Remove a given file from the filesystem and the program
    Remove { file: Utf8PathBuf },
    /// Cache an unknown manifest file for future use
    Cache { file: Utf8PathBuf },
    /// Validate a manually changed or potentially corrupted file
    Validate { file: Utf8PathBuf },
}

#[derive(Args, Clone, Default)]
pub struct CreateArgs {
    /// Create the file with given data without the interactive mode
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub non_interactive: bool,
    /// Directory of the project
    #[arg(short, long)]
    pub directory: Option<Utf8PathBuf>,
    /// Title of the project
    #[arg(short, long)]
    pub title: Option<String>,
    /// Categories of the project (space seperated), syntax: <CATEGORY> ...
    #[arg(short, long, value_parser, num_args = 1..)]
    pub categories: Vec<String>,
    /// Languages of the project (space seperated), syntax: <NAME>:<VERSION> ...
    #[arg(short, long, value_parser, num_args = 1..)]
    pub languages: Vec<String>,
    /// Preferred IDE of the project, syntax: <NAME>:<VERSION>
    #[arg(short, long)]
    pub preferred_ide: Option<String>,
    /// Build systems of the project (space seperated), syntax: <NAME>:<VERSION> ...
    #[arg(short, long, value_parser, num_args = 1..)]
    pub build_systems: Vec<String>,
    /// Description of the project
    #[arg(short = 'D', long)]
    pub description: Option<String>,
    /// Repository URL of the project
    #[arg(short, long)]
    pub repository_url: Option<String>,
}

#[cfg(test)]
pub mod tests {
    /// Skip a test if stdout is used.
    /// See test documentation in [`cli::prompts`] for more information
    pub fn skip_if_stdout() {
        if atty::is(atty::Stream::Stdout) {
            eprintln!("Skipping test with stdout when not using nextest...");
            panic!("IO error: not a terminal");
        }
    }
}
