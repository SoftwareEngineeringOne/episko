use std::str::FromStr;

use camino::{Utf8Path, Utf8PathBuf};
use cli::{
    prompts::{
        build_systems_prompt, categories_prompt, description_prompt, directory_prompt, ide_prompt,
        languages_prompt, repository_url_prompt, title_prompt,
    },
    CreateArgs,
};
use color_eyre::{eyre::eyre, Result};
use episkos_lib::{
    files::File,
    metadata::{builder::ApplyIf, BuildSystem, Category, Ide, Language, Metadata, MetadataBuilder},
};

pub mod cli;

pub fn create(args: CreateArgs) -> Result<()> {
    let builder = Metadata::builder();

    let builder = match args.non_interactive {
        false => run_interactive_creation(args, builder)?,
        true => run_non_interactiv_creation(args, builder)?,
    };

    let metadata = builder.build()?;

    metadata.write_file(metadata.directory())?;

    Ok(())
}

fn run_interactive_creation(args: CreateArgs, builder: MetadataBuilder) -> Result<MetadataBuilder> {
    println!("If values have been provided via flags, they will be set as the defaults!");
    let directory = directory_prompt(args.directory)?;
    let title = title_prompt(args.title)?;
    let description = description_prompt(args.description)?;
    let categories = categories_prompt(args.categories)?;
    let languages = languages_prompt(args.languages)?;
    let build_systems = build_systems_prompt(args.build_systems)?;
    let preferred_ide = ide_prompt(args.preferred_ide)?;
    let repository_url = repository_url_prompt(args.repository_url)?;

    Ok(builder
        .directory_path(directory.as_std_path())
        .title(&title)
        .categories(categories)
        .languages(languages)
        .build_systems(build_systems)
        .apply_if(description.as_deref(), MetadataBuilder::description)
        .apply_if(preferred_ide, MetadataBuilder::preffered_ide)
        .apply_if(repository_url.as_deref(), MetadataBuilder::repository_url))
}

fn run_non_interactiv_creation(
    args: CreateArgs,
    builder: MetadataBuilder,
) -> Result<MetadataBuilder> {
    // Extract ide if given
    let preferred_ide = args
        .preferred_ide
        .as_deref()
        .map(Ide::from_str)
        .transpose()?;

    // Assign non-vec properties
    let mut builder = builder
        .apply_if(
            args.directory.as_deref().map(Utf8Path::as_std_path),
            MetadataBuilder::directory_path,
        )
        .apply_if(args.title.as_deref(), MetadataBuilder::title)
        .apply_if(preferred_ide, MetadataBuilder::preffered_ide)
        .apply_if(args.description.as_deref(), MetadataBuilder::description)
        .apply_if(
            args.repository_url.as_deref(),
            MetadataBuilder::repository_url,
        );

    // Prepare vec properties
    let categories = args
        .categories
        .iter()
        .map(|el| Category::from_str(el))
        .filter_map(Result::ok)
        .collect();

    // for loop used, to allow  for returning Err on invalid argument
    let languages: Vec<Language> = args
        .languages
        .into_iter()
        .map(|arg| {
            let tuple = arg.parse_tuple()?;
            Ok(tuple.try_into()?)
        })
        .collect::<Result<_>>()?;

    let build_systems: Vec<BuildSystem> = args
        .build_systems
        .into_iter()
        .map(|bs_arg| {
            let tuple = bs_arg.parse_tuple()?;
            Ok(tuple.try_into()?)
        })
        .collect::<Result<_>>()?;

    // Assign vec properties
    builder = builder
        .categories(categories)
        .languages(languages)
        .build_systems(build_systems);

    Ok(builder)
}

pub fn remove(file: &Utf8PathBuf) -> Result<()> {
    Ok(Metadata::remove_file(file.as_std_path())?)
}

pub fn add(_file: &Utf8PathBuf) -> Result<()> {
    // Ok(Metadata::from_file(file.as_std_path())?)
    todo!()
}

pub fn validate(file: &Utf8PathBuf) -> Result<()> {
    Ok(Metadata::validate_file(file.as_std_path())?)
}

pub trait ComplexArg {
    fn parse_tuple(self) -> Result<(String, String)>;
}

impl ComplexArg for String {
    fn parse_tuple(self) -> Result<(String, String)> {
        let parts: Vec<&str> = self.split(":").collect();

        // Name has to be given, version is optional
        match parts.len() {
            1 => Ok((parts[0].to_string(), String::default())),
            2 => Ok((parts[0].to_string(), parts[1].to_string())),
            _ => Err(eyre!("invalid input")),
        }
    }
}
