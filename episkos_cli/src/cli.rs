use camino::Utf8PathBuf;
use clap::{ArgAction, Args, Parser, Subcommand};



/// CLI interface of Episkos
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
    Create ( CreateArgs ),
    /// Remove a given file from the filesystem and the program
    Remove { file: Utf8PathBuf },
    /// Add a given file to the program
    Add { file: Utf8PathBuf },
    /// Check a manual changed file
    Validate { file: Utf8PathBuf },
}

#[derive(Args)]
pub struct CreateArgs {
        /// Create the file with given data without the interactive mode
        #[arg(short, long, action=ArgAction::SetFalse)]
        pub non_interactive: bool,
        /// Directory of the project
        #[arg(short, long)]
        pub directory: Option<Utf8PathBuf>,
        /// Title of the project
        #[arg(short, long)]
        pub title: Option<String>,
        /// Categories of the project
        #[arg(short, long)]
        pub categories: Option<Vec<String>>,
        /// Languages of the project, syntax: <NAME>:<VERSION> ...
        #[arg(short, long)]
        pub languages: Option<Vec<String>>,
        /// Preferred IDE of the project, syntax: <NAME>:<VERSION>
        #[arg(short, long)]
        pub preferred_ide: Option<String>,
        /// Build systems of the project, syntax: <NAME>:<VERSION> ...
        #[arg(short, long)]
        pub build_systems: Option<Vec<String>>,
        /// Description of the project
        #[arg(short = 'D', long)]
        pub description: Option<String>,
        /// Repository URL of the project
        #[arg(short, long)]
        pub repository_url: Option<String>,
}
