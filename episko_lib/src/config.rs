use std::path::PathBuf;

pub mod config_controller;

pub struct Config {
    pub directories_to_load: Vec<PathBuf>,
    pub files_to_load: Vec<PathBuf>,
    pub database_directory: PathBuf,
}