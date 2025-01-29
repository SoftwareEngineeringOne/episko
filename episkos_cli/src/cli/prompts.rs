//! Prompts for interactive creation mode
//! 
//! This module contains the prompts for the interactive creation mode.
//! 
//! ## Default values
//! A default value (value from a flag) disables the prompt and is used instead of some input.

use std::str::FromStr;

use crate::ComplexArg;
use camino::Utf8PathBuf;
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use episkos_lib::metadata::{BuildSystem, Category, Ide, Language};

/// Maximum number of input prompts for vec data
const MAX_ROUNDS: i8 = 25;

/// Specific prompt for the directory
pub fn directory_prompt(default: Option<Utf8PathBuf>) -> Result<Utf8PathBuf> {
    if let Some(dir) = default {
        return Ok(dir);
    }
    text_prompt("Directory", false, default)
}

/// Specific prompt for the title
pub fn title_prompt(default: Option<String>) -> Result<String> {
    if let Some(title) = default {
        return Ok(title);
    }
    text_prompt("Title", false, default)
}

/// Specific prompt for the description
pub fn description_prompt(default: Option<String>) -> Result<Option<String>> {
    if let Some(description) = default {
        return Ok(Some(description));
    }
    optional_text_prompt("Description", default)
}

/// Specific prompt for the categories
pub fn categories_prompt(defaults: Vec<String>) -> Result<Vec<Category>> {
    let mut categories = Vec::with_capacity(defaults.len());
    let mut defaults = defaults.iter();

    if defaults.len() > 0 {
        for default in defaults {
            categories.push(Category::from_str(default)?);
        }
        return Ok(categories);
    }

    for i in 1..MAX_ROUNDS {
        let default = defaults.next().map(|el| el.to_string());
        let input: String = text_prompt(&format!("Category {}", i), true, default)?;

        if input.is_empty() {
            break;
        }

        categories.push(Category::from_str(&input)?);
    }
    Ok(categories)
}

/// Specific prompt for the languages
pub fn languages_prompt(defaults: Vec<String>) -> Result<Vec<Language>> {
    looping_prompt_with_version("Language", defaults)
}

/// Specific prompt for the build systems
pub fn build_systems_prompt(defaults: Vec<String>) -> Result<Vec<BuildSystem>> {
    looping_prompt_with_version("Build System", defaults)
}

/// Specific prompt for the ide
pub fn ide_prompt(default: Option<String>) -> Result<Option<Ide>> {
    if let Some(ide) = default {
        return Ok(Some(Ide::from_str(&ide)?));
    }
    optional_text_prompt("Preferred Ide", default)
}

/// Specific prompt for the repository url
pub fn repository_url_prompt(default: Option<String>) -> Result<Option<String>> {
    if let Some(url) = default {
        return Ok(Some(url));
    }
    optional_text_prompt("Repository Url", default)
}

/// Universal prompt for standard text
fn text_prompt<T>(prompt: &str, allow_empty: bool, default: Option<T>) -> Result<T>
where
    T: Clone + FromStr + std::fmt::Display,
    <T as FromStr>::Err: std::fmt::Display,
{
    let theme = ColorfulTheme::default();

    let mut input = Input::<T>::with_theme(&theme)
        .with_prompt(prompt)
        .allow_empty(allow_empty);

    if let Some(default) = default {
        input = input.default(default);
    }

    Ok(input.interact_text()?)
}

/// Universal prompt for optional standard text
fn optional_text_prompt<T>(prompt: &str, default: Option<String>) -> Result<Option<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let data: String = text_prompt(prompt, true, default)?;

    match data.is_empty() {
        true => Ok(None),
        false => Ok(Some(T::from_str(&data)?)),
    }
}

/// Universal prompt for multiple inputs with version
fn looping_prompt_with_version<T>(prompt: &str, defaults: Vec<String>) -> Result<Vec<T>>
where
    T: TryFrom<(String, String)>,
    // Accept every type of error for color_eyre
    <T as TryFrom<(String, String)>>::Error: std::error::Error + Send + Sync + 'static,
{
    let mut data = vec![];

    if !defaults.is_empty() {
        let mut data: Vec<T> = Vec::with_capacity(defaults.len());
        for default in defaults.iter() {
            data.push(T::try_from(default.clone().parse_tuple()?)?)
        }
        return Ok(data);
    }

    for i in 1..MAX_ROUNDS {
        let name: String = text_prompt(&format!("{} {} Name", prompt, i), true, None)?;

        if name.is_empty() {
            break;
        }

        let version: String = text_prompt(&format!("{} {} Version", prompt, i), true, None)?;
        data.push((name, version).try_into()?)
    }
    Ok(data)
}
