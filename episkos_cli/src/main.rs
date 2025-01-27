use clap::Parser;
use color_eyre::Result;
use episkos_cli::cli;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Create(create_args) => {
            episkos_cli::create(create_args).unwrap();
        }
        cli::Commands::Remove { file } => {
            episkos_cli::remove(&file)?;
        }
        cli::Commands::Add { file } => {
            episkos_cli::add(&file)?;
        }
        cli::Commands::Validate { file } => {
            episkos_cli::validate(&file)?;
        }
    }
    Ok(())
}
