use clap::Parser;
use episkos_cli::cli;



fn main() {
    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Create( mut create_args ) => {
            episkos_cli::create(&mut create_args).unwrap();
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
