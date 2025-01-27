use clap::Parser;
use episkos_cli::cli;



fn main() {
    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Create( mut createArgs ) => {
            episkos_cli::create(&mut createArgs).unwrap();
        }
        cli::Commands::Remove { file } => {
            episkos_cli::remove(&file);
        }
        cli::Commands::Add { file } => {
            episkos_cli::add(&file);
        }
        cli::Commands::Validate { file } => {
            episkos_cli::validate(&file);
        }
    }
}
