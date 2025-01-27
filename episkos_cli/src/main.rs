use clap::Parser;
use color_eyre::Result;
use episkos_cli::cli;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Create(create_args) => {
            episkos_cli::create_manifest(create_args).unwrap();
        }
        cli::Commands::Remove { file } => {
            episkos_cli::remove_manifest(&file)?;
        }
        cli::Commands::Cache { file } => {
            episkos_cli::cache_manifest(&file)?;
        }
        cli::Commands::Validate { file } => {
            episkos_cli::validate_manifest(&file)?;
        }
    }
    Ok(())
}
