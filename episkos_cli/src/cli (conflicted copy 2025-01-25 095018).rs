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
    Create (
        CreateArgs
        /*/// Create the file with given data without the interactive mode
        #[arg(short, long, action=ArgAction::SetFalse)]
        non_interactive: bool,
        /// Directory of the project
        #[arg(short, long)]
        directory: Option<Utf8PathBuf>,
        /// Title of the project
        #[arg(short, long)]
        title: Option<String>,
        /// Categories of the project
        #[arg(short, long)]
        categories: Option<Vec<String>>,
        /// Languages of the project, syntax: <NAME>:<VERSION> ...
        #[arg(short, long)]
        languages: Option<Vec<String>>,
        /// Preferred IDE of the project, syntax: <NAME>:<VERSION>
        #[arg(short, long)]
        preferred_ide: Option<String>,
        /// Build systems of the project, syntax: <NAME>:<VERSION> ...
        #[arg(short, long)]
        build_systems: Option<Vec<String>>,
        /// Description of the project
        #[arg(short = 'D', long)]
        description: Option<String>,
        /// Repository URL of the project
        #[arg(short, long)]
        repository_url: Option<String>,*/
    ),
    /// Remove a given file from the filesystem and the program
    Remove { file: Utf8PathBuf },
    /// Add a given file to the program
    Add { file: Utf8PathBuf },
    /// Check a manual changed file
    Validate { file: Utf8PathBuf },
}

#[derive(Args)]
struct CreateArgs {

        /// Create the file with given data without the interactive mode
        #[arg(short, long, action=ArgAction::SetFalse)]
        non_interactive: bool,
        /// Directory of the project
        #[arg(short, long)]
        directory: Option<Utf8PathBuf>,
        /// Title of the project
        #[arg(short, long)]
        title: Option<String>,
        /// Categories of the project
        #[arg(short, long)]
        categories: Option<Vec<String>>,
        /// Languages of the project, syntax: <NAME>:<VERSION> ...
        #[arg(short, long)]
        languages: Option<Vec<String>>,
        /// Preferred IDE of the project, syntax: <NAME>:<VERSION>
        #[arg(short, long)]
        preferred_ide: Option<String>,
        /// Build systems of the project, syntax: <NAME>:<VERSION> ...
        #[arg(short, long)]
        build_systems: Option<Vec<String>>,
        /// Description of the project
        #[arg(short = 'D', long)]
        description: Option<String>,
        /// Repository URL of the project
        #[arg(short, long)]
        repository_url: Option<String>,

}