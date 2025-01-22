use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};



/// CLI interface of Episkos
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new manifest file
    Create {
        /// Create the file with given data without the interactive mode
        #[arg(short, long)]
        data: Option<String>,
    },
    /// Remove a given file from the filesystem and the program
    Remove { file: Utf8PathBuf },
    /// Add a given file to the program
    Add { file: Utf8PathBuf },
    /// Check a manual changed file
    Validate { file: Utf8PathBuf },
}