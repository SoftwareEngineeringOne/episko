#![deny(clippy::pedantic)]
//! # Main
//!
//! This module contains the start of the program.
//!
//! ## Structure
//! The library is structured into the following modules:
//! - cli
//! - creation
//! - removal
//! - validation
//!
//! Detailed documentation can be found within each module.

use clap::Parser;
use color_eyre::Result;
use episko_cli::cli;
use episko_lib::config::config_handler::ConfigHandler;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = cli::Cli::parse();
    let config_handler = ConfigHandler::new()?;

    match args.command {
        cli::Commands::Create(create_args) => {
            episko_cli::create_manifest(create_args, &config_handler).await?;
        }
        cli::Commands::Remove { file } => {
            episko_cli::remove_manifest(&file, &config_handler).await?;
        }
        cli::Commands::Cache { file } => {
            episko_cli::cache_manifest(&file, &config_handler).await?;
        }
        cli::Commands::Validate { file } => {
            episko_cli::validate_manifest(&file, &config_handler).await?;
        }
    }
    Ok(())
}
