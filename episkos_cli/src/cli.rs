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
        non_interactive: Option<bool>,
        /// Id of the project
        #[arg(short, long)]
        id: Option<String>,
        /// Directory of the project
        #[arg(short, long)]
        directory: Option<Utf8PathBuf>,
        /// Title of the project
        #[arg(short, long)]
        title: Option<String>,
        /// Categories of the project
        #[arg(short, long)]
        categories: Option<Vec<String>>,
        /// Languages of the project
        #[arg(short, long)]
        languages: Option<Vec<String>>,//<Language>>,
        /// Preferred IDE of the project
        #[arg(short, long)]
        preferred_ide: Option<String>,//<Ide>,
        /// Build system of the project
        #[arg(short, long)]
        build_systems: Option<Vec<String>>,//<BuildSystem>>,
        /// Description of the project
        #[arg(short = 'D', long)]
        description: Option<String>,
        /// Repository URL of the project
        #[arg(short, long)]
        repository_url: Option<String>,
    },
    /// Remove a given file from the filesystem and the program
    Remove { file: Utf8PathBuf },
    /// Add a given file to the program
    Add { file: Utf8PathBuf },
    /// Check a manual changed file
    Validate { file: Utf8PathBuf },
}
