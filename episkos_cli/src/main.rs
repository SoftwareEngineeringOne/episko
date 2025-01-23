use clap::Parser;
use episkos_cli::cli;



fn main() {
    let args = cli::Args::parse();

    match &args.command {
        cli::Commands::Create { non_interactive, id, directory, title, categories, languages, preferred_ide, build_systems, description , repository_url } => {
            // TODO: Take all arguments into a struct and give it to the create function
            //episkos_cli::create(data.clone());
        }
        cli::Commands::Remove { file } => {
            episkos_cli::remove(file.clone());
        }
        cli::Commands::Add { file } => {
            episkos_cli::add(file.clone());
        }
        cli::Commands::Validate { file } => {
            episkos_cli::validate(file.clone());
        }
    }
}
