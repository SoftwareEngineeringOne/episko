use std::{error::Error, path::Path};
use dialoguer::Input;
use camino::Utf8PathBuf;
use cli::CreateArgs;
use episkos_lib::{files::File, metadata::Metadata};

pub mod cli;



pub fn create( args: &CreateArgs ) -> Result<(), Box<dyn Error>> {
    // TODO: Get all information from the user in a interactive mode or given data and create a new manifest
    /*match args.non_interactive  {
        false => todo!(),
        true => todo!()
    }*/

    if args.non_interactive {
        if args.directory.is_none() {
            println!("Error: Directory is missing!")
        }
        if args.title.is_none() {
            println!("Error: Title is missing!")
        }
    } else {
        /*args.directory = Input::<Utf8PathBuf>::new()
            .with_prompt("Your directory?")
            .interact_text()
            .unwrap();
        args.title = Some(Input::<String>::new()
            .with_prompt("Your title?")
            .interact_text()
            .unwrap());*/
    }

    let metadata = Metadata::builder()
        .directory(args.directory.as_ref().expect("Error: Directory is missing!").as_std_path())?
        .title(args.title.as_ref().expect("Error: Title is missing!"))
        .build()?;

    /*if args.categories.is_some() {metadata.update().categories(args.categories);}
    if args.languages.is_some() {metadata.update().languages(args.languages);}
    if args.preferred_ide.is_some() {metadata.update().preffered_ide(args.preferred_ide);}
    if args.build_systems.is_some() {metadata.update().build_systems(args.build_systems);}
    if args.description.is_some() {metadata.update().description(args.description.as_ref().expect(""));}
    if args.repository_url.is_some() {metadata.update().repository_url(args.repository_url.as_ref().expect(""));}*/

    metadata.write_file(&metadata.directory())?;

    metadata.write_file(&metadata.directory())?;

    // TODO: TO BE REMOVED!
    let read_metadata = Metadata::from_file(Path::new("."))?;
    println!("Read: {:#?}", read_metadata);

    Ok(())
}

pub fn remove( file: &Utf8PathBuf ) {
    // TODO: Call the method to remove the file
}

pub fn add( file: &Utf8PathBuf ) {
    // TODO: Call to add the file to the system
}

pub fn validate( file: &Utf8PathBuf ) {
    // TODO: Call to validate a given file
}
