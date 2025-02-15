//! # Creation of a new manifest
//!
//! This module contains the first level of code for the creation of a manifest and is directly used by the the create command.

use std::str::FromStr;

use crate::connect_to_db;

use super::cli::{
    prompts::{
        build_systems_prompt, categories_prompt, description_prompt, directory_prompt, ide_prompt,
        languages_prompt, repository_url_prompt, title_prompt,
    },
    CreateArgs,
};
use super::ComplexArg;
use camino::Utf8Path;
use color_eyre::Result;
use episko_lib::{
    config::Config,
    files::File,
    metadata::{builder::ApplyIf, BuildSystem, Category, Ide, Language, Metadata, MetadataBuilder},
};

/// Create a manifest based on the given cli arguments
///
/// # Errors
/// - Propogates errors from [`run_non_interactiv_creation`]
/// - Propogates errors from [`run_interactive_creation`]
/// - Propogates errors from [`connect_to_db`]
/// - [`color_eyre::Report`] if [`MetadataBuilder::build`] fails
/// - [`color_eyre::Report`] if [`Metadata::write_to_db`] fails
/// - [`color_eyre::Report`] if [`Metadata::write_file`] fails
pub async fn create_manifest(args: CreateArgs, config: &Config) -> Result<()> {
    let builder = Metadata::builder();

    let builder = if args.non_interactive {
        run_non_interactive_creation(args, builder)?
    } else {
        run_interactive_creation(args, builder)?
    };

    let metadata = builder.build()?;

    let db = connect_to_db(config).await?;
    metadata.write_to_db(&db).await?;
    metadata.write_file(metadata.directory())?;

    Ok(())
}

/// Create a manifest with interactive prompts for missing attributes
///
/// # Errors
/// - Propogates errors from each prompt:
///     - [`directory_prompt`]
///     - [`title_prompt`]
///     - [`description_prompt`]
///     - [`categories_prompt`]
///     - [`languages_prompt`]
///     - [`build_systems_prompt`]
///     - [`ide_prompt`]
///     - [`repository_url_prompt`]
fn run_interactive_creation(args: CreateArgs, builder: MetadataBuilder) -> Result<MetadataBuilder> {
    println!("Flag passed values will not be prompted!");
    let directory = directory_prompt(args.directory)?;
    let title = title_prompt(args.title)?;
    let description = description_prompt(args.description)?;
    let categories = categories_prompt(&args.categories)?;
    let languages = languages_prompt(&args.languages)?;
    let build_systems = build_systems_prompt(&args.build_systems)?;
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

/// Create a manifest with interactive prompts for missing attributes
///
/// # Errors
/// - [`color_eyre::Report`] when [`Ide::from_str`] fails
/// - [`color_eyre::Report`] when [`ComplexArg::parse_tuple`] fails
///     - This is called for [`Language`] and [`BuildSystem`]
fn run_non_interactive_creation(
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
        .map(|el| {
            let tuple = el.parse_tuple()?;
            Ok(tuple.try_into()?)
        })
        .collect::<Result<_>>()?;

    let build_systems: Vec<BuildSystem> = args
        .build_systems
        .into_iter()
        .map(|el| {
            let tuple = el.parse_tuple()?;
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
