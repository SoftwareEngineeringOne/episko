use camino::Utf8PathBuf;
use cli::CreateArgs;
use dialoguer::Input;
use episkos_lib::{
    files::File,
    metadata::{BuildSystem, Category, Ide, Language, Metadata},
};
use std::error::Error;

pub mod cli;

pub fn create(args: &mut CreateArgs) -> Result<(), Box<dyn Error>> {
    if args.non_interactive {
        if args.directory.is_none() {
            println!("Error: Directory is missing!")
        }
        if args.title.is_none() {
            println!("Error: Title is missing!")
        }
    } else {
        interactive_input(args);
    }

    let mut builder = Metadata::builder()
        .directory(".")
        .title(args.title.as_ref().expect("Error: Title is missing!"));

    builder = builder.categories(args.categories.iter().map(|el| Category::new(el)).collect());
    builder = builder.languages(
        args.languages
            .iter()
            .map(|el| {
                let mut split = el.split(':');
                let name = split.next();
                let version = split.next();
                if version.is_some() {
                    return Language::with_version(name.expect(""), version.expect(""));
                } else {
                    return Language::new(name.expect(""));
                }
            })
            .collect(),
    );
    if args.preferred_ide.is_some() {
        builder = builder.preffered_ide(Ide::new(args.preferred_ide.as_ref().expect("")));
    }
    builder = builder.build_systems(
        args.build_systems
            .iter()
            .map(|el| {
                let mut split = el.split(':');
                let name = split.next();
                let version = split.next();
                if version.is_some() {
                    return BuildSystem::with_version(name.expect(""), version.expect(""));
                } else {
                    return BuildSystem::new(name.expect("msg"));
                }
            })
            .collect(),
    );
    if args.description.is_some() {
        builder = builder.description(args.description.as_ref().expect(""));
    }
    if args.repository_url.is_some() {
        builder = builder.repository_url(args.repository_url.as_ref().expect(""));
    }

    let metadata = builder.build()?;

    metadata.write_file(&metadata.directory())?;

    Ok(())
}

pub fn remove(file: &Utf8PathBuf) {
    let _ = Metadata::remove_file(file.as_std_path());
}

pub fn add(file: &Utf8PathBuf) {
    let _ = Metadata::from_file(file.as_std_path());
}

pub fn validate(file: &Utf8PathBuf) {
    let _ = Metadata::validate_file(file.as_std_path());
}

fn interactive_input(args: &mut CreateArgs) {
    if args.directory.is_none() {
        args.directory = Some(
            Input::new()
                .with_prompt("Directory")
                .interact_text()
                .unwrap(),
        );
    }
    if args.title.is_none() {
        args.title = Some(Input::new().with_prompt("Title").interact_text().unwrap());
    }
    if args.categories.is_empty() {
        loop {
            let input: String = Input::new()
                .with_prompt("Categories")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            if input.is_empty() {
                break;
            };
            args.categories.push(input);
        }
    }
    if args.languages.is_empty() {
        loop {
            let input: String = Input::new()
                .with_prompt("Languages")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            if input.is_empty() {
                break;
            };
            args.languages.push(input);
        }
    }
    if args.preferred_ide.is_none() {
        args.preferred_ide = Some(
            Input::new()
                .with_prompt("Preferred IDE")
                .allow_empty(true)
                .interact_text()
                .unwrap(),
        );
    }
    if args.build_systems.is_empty() {
        loop {
            let input: String = Input::new()
                .with_prompt("Build systems")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            if input.is_empty() {
                break;
            };
            args.build_systems.push(input);
        }
    }
    if args.description.is_none() {
        args.description = Some(
            Input::new()
                .with_prompt("Description")
                .allow_empty(true)
                .interact_text()
                .unwrap(),
        );
    }
    if args.repository_url.is_none() {
        args.repository_url = Some(
            Input::new()
                .with_prompt("Repository url")
                .allow_empty(true)
                .interact_text()
                .unwrap(),
        );
    }
}
