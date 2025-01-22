use clap::Parser;

mod cli;
mod lib;



fn main() {
    let args = cli::Args::parse();

    match &args.command {
        cli::Commands::Create { data } => {
            lib::create(data.clone());
        }
        cli::Commands::Remove { file } => {
            lib::remove(file.clone());
        }
        cli::Commands::Add { file } => {
            lib::add(file.clone());
        }
        cli::Commands::Validate { file } => {
            lib::validate(file.clone());
        }
    }
}
