use std::{error::Error, io::Split, path::Path};
use clap::builder::{self, Str};
use dialoguer::Input;
use camino::Utf8PathBuf;
use cli::CreateArgs;
use episkos_lib::{files::File, metadata::{BuildSystem, Category, Ide, Language, Metadata}};

pub mod cli;



pub fn create(args: &mut CreateArgs ) -> Result<(), Box<dyn Error>> {
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
        args.directory = Some(Input::<Utf8PathBuf>::new()
            .with_prompt("Directory")
            .interact_text()
            .unwrap());
        args.title = Some(Input::new()
            .with_prompt("Title")
            .interact_text()
            .unwrap());
        loop {
            let input: String = Input::new()
                .with_prompt("Categories")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            if input.is_empty() { break };
            args.categories.push(input); 
        }
        loop {
            let input: String = Input::new()
                .with_prompt("Languages")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            if input.is_empty() { break };
            args.languages.push(input);
        }
        args.preferred_ide = Some(Input::new()
            .with_prompt("Preferred IDE")
            .allow_empty(true)
            .interact_text()
            .unwrap());
        loop {
            let input: String = Input::new()
                .with_prompt("Build systems")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            if input.is_empty() { break };
            args.build_systems.push(input);
        }
        args.description = Some(Input::new()
            .with_prompt("Description")
            .allow_empty(true)
            .interact_text()
            .unwrap());
        args.repository_url = Some(Input::new()
            .with_prompt("Repository url")
            .allow_empty(true)
            .interact_text()
            .unwrap());
    }

    let mut builder = Metadata::builder()
        .directory(".")
        .title(args.title.as_ref().expect("Error: Title is missing!"));
 
    builder = builder.categories(
        args.categories
        .iter()
        .map(|el| Category::new(el))
        .collect()
    );

    for i in args.categories.iter() {
        builder = builder.add_category(&i);
    }

    builder = builder.languages(
        args.languages
        .iter()
        .map(|el|{
            let mut split = el.split(':');
            let name = split.next();
            let version = split.next();
            Language::with_version(name.expect(""), version.expect(""))
        })
        .collect()
    );
    if args.preferred_ide.is_some() {builder = builder.preffered_ide(Ide::new(args.preferred_ide.as_ref().expect("")));}
    for i in args.build_systems.clone() {
        let mut split = i.split(':');
        let name = split.next();
        let version = split.next();
        builder = builder.add_build_system(BuildSystem::with_version(name.expect(""), version.expect("")));
    }
    if args.description.is_some() {builder = builder.description(args.description.as_ref().expect(""));}
    if args.repository_url.is_some() {builder = builder.repository_url(args.repository_url.as_ref().expect(""));}

    let metadata = builder.build()?;

    metadata.write_file(&metadata.directory())?;

    // TODO: TO BE REMOVED!
    let read_builder = Metadata::from_file(Path::new("."))?;
    println!("Read: {:#?}", read_builder);

    Ok(())
}

pub fn remove( file: &Utf8PathBuf ) {
    // TODO: Call the method to remove the file
    // Call remove
}

pub fn add( file: &Utf8PathBuf ) {
    // TODO: Call to add the file to the system
    // Call from_file
}

pub fn validate( file: &Utf8PathBuf ) {
    // TODO: Call to validate a given file
    // Call validate
}
