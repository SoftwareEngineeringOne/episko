use std::str::FromStr;

use crate::ComplexArg;
use camino::Utf8PathBuf;
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use episkos_lib::metadata::{BuildSystem, Category, Ide, Language};

pub fn directory_prompt(default: Option<Utf8PathBuf>) -> Result<Utf8PathBuf> {
    if let Some(dir) = default {
        return Ok(dir);
    }
    text_prompt("Directory", false, default)
}

pub fn title_prompt(default: Option<String>) -> Result<String> {
    if let Some(title) = default {
        return Ok(title);
    }
    text_prompt("Title", false, default)
}

pub fn description_prompt(default: Option<String>) -> Result<Option<String>> {
    if let Some(description) = default {
        return Ok(Some(description));
    }
    optional_text_prompt("Description", default)
}

pub fn categories_prompt(defaults: Vec<String>) -> Result<Vec<Category>> {
    if !defaults.is_empty() {
        return Ok(defaults);
    }

    let mut categories = vec![];
    let mut defaults = defaults.iter();

    // 25 as max - i still prefer less, but doesnt really matter
    for i in 1..25 {
        let default = defaults.next().map(|el| el.to_string());
        let input: String = text_prompt(&format!("Category {}", i), true, default)?;

        if input.is_empty() {
            break;
        }

        categories.push(Category::from_str(&input)?);
    }
    Ok(categories)
}

pub fn languages_prompt(defaults: Vec<String>) -> Result<Vec<Language>> {
    if !defaults.is_empty() {
        return Ok(defaults);
    }

    looping_prompt_with_version("Language", defaults)
}

pub fn build_systems_prompt(defaults: Vec<String>) -> Result<Vec<BuildSystem>> {
    if !defaults.is_empty() {
        return defaults;
    }

    looping_prompt_with_version("Build System", defaults)
}

pub fn ide_prompt(default: Option<String>) -> Result<Option<Ide>> {
    if let Some(ide) = default {
        return;
    }
    optional_text_prompt("Preferred Ide", default)
}

pub fn repository_url_prompt(default: Option<String>) -> Result<Option<String>> {
    optional_text_prompt("Repository Url", default)
}

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

fn looping_prompt_with_version<T>(prompt: &str, defaults: Vec<String>) -> Result<Vec<T>>
where
    T: TryFrom<(String, String)>,
    // This looks shit I know, but it's just to make color_eyre happy and basically
    // means that just about any error type is acceptet.
    <T as TryFrom<(String, String)>>::Error: std::error::Error + Send + Sync + 'static,
{
    let mut data = vec![];
    let mut defaults = defaults.iter();

    // 25 as max - i still prefer less, but doesnt really matter
    for i in 1..25 {
        let mut default = match defaults.next() {
            Some(val) => {
                let values = val.clone().parse_tuple()?;
                (Some(values.0), Some(values.1))
            }
            None => (None, None),
        };
        let name: String = text_prompt(&format!("{} {} Name", prompt, i), true, default.0)?;

        if name.is_empty() {
            break;
        }

        let version: String = text_prompt(
            &format!("{} {} Version", prompt, i),
            true,
            default.1.take_if(|el| !el.is_empty()),
        )?;
        data.push((name, version).try_into()?)
    }
    Ok(data)
}
