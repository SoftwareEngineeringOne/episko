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
use episko_lib::metadata::{BuildSystem, Category, Ide, Language};

/// Maximum number of input prompts for vec data
const MAX_ROUNDS: i8 = 25;

/// Specific prompt for the directory
///
/// # Errors
/// - Propogates errors from [`text_prompt`]
pub fn directory_prompt(default: Option<Utf8PathBuf>) -> Result<Utf8PathBuf> {
    if let Some(dir) = default {
        return Ok(dir);
    }
    text_prompt("Directory", false, default)
}

/// Specific prompt for the title
///
/// # Errors
/// - Propogates errors from [`text_prompt`]
pub fn title_prompt(default: Option<String>) -> Result<String> {
    if let Some(title) = default {
        return Ok(title);
    }
    text_prompt("Title", false, default)
}

/// Specific prompt for the description
///
/// # Errors
/// - Propogates errors from [`optional_text_prompt`]
pub fn description_prompt(default: Option<String>) -> Result<Option<String>> {
    if let Some(description) = default {
        return Ok(Some(description));
    }
    optional_text_prompt("Description", default)
}

/// Specific prompt for the categories
///
/// # Errors
/// - Propogates errors from [`text_prompt`]
/// - [`color_eyre::Report`] when creating a [`Category`] fails.
pub fn categories_prompt(defaults: &[String]) -> Result<Vec<Category>> {
    let mut categories = Vec::with_capacity(defaults.len());
    let mut defaults = defaults.iter();

    if defaults.len() > 0 {
        for default in defaults {
            categories.push(Category::from_str(default)?);
        }
        return Ok(categories);
    }

    for i in 1..MAX_ROUNDS {
        let default = defaults.next().map(ToString::to_string);
        let input: String = text_prompt(&format!("Category {i}"), true, default)?;

        if input.is_empty() {
            break;
        }

        categories.push(Category::from_str(&input)?);
    }
    Ok(categories)
}

/// Specific prompt for the languages
///
/// # Errors
/// - Propogates errors from [`looping_prompt_with_version`]
pub fn languages_prompt(defaults: &[String]) -> Result<Vec<Language>> {
    looping_prompt_with_version("Language", defaults)
}

/// Specific prompt for the build systems
///
/// # Errors
/// - Propogates errors from [`looping_prompt_with_version`]
pub fn build_systems_prompt(defaults: &[String]) -> Result<Vec<BuildSystem>> {
    looping_prompt_with_version("Build System", defaults)
}

/// Specific prompt for the ide
///
/// # Errors
/// - Propogates errors from [`optional_text_prompt`]
/// - [`color_eyre::Report`] when creating an [`Ide`] instance from the default
///   value fails.
pub fn ide_prompt(default: Option<String>) -> Result<Option<Ide>> {
    if let Some(ide) = default {
        return Ok(Some(Ide::from_str(&ide)?));
    }
    optional_text_prompt("Preferred Ide", default)
}

/// Specific prompt for the repository url
///
/// # Errors
/// - Propogates errors from [`optional_text_prompt`]
pub fn repository_url_prompt(default: Option<String>) -> Result<Option<String>> {
    if let Some(url) = default {
        return Ok(Some(url));
    }
    optional_text_prompt("Repository Url", default)
}

/// Universal prompt for standard text
///
/// # Errors
/// - [`color_eyre::Report`] when [`Input::interact_text`] fails
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
///
/// # Errors
/// - Propogates errors from [`text_prompt`]
/// - [`color_eyre::Report`] when [`Input::interact_text`] fails
/// - [`color_eyre::Report`] when [`FromStr::from_str`] fails
fn optional_text_prompt<T>(prompt: &str, default: Option<String>) -> Result<Option<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let data: String = text_prompt(prompt, true, default)?;

    if data.is_empty() {
        Ok(None)
    } else {
        Ok(Some(T::from_str(&data)?))
    }
}

/// Universal prompt for multiple inputs with version
///
/// # Errors
/// - Propogates errors from [`text_prompt`]
/// - [`color_eyre::Report`] when [`ComplexArg::parse_tuple`] fails
/// - [`color_eyre::Report`] when [`TryFrom<(String, String)>`] fails
///
fn looping_prompt_with_version<T>(prompt: &str, defaults: &[String]) -> Result<Vec<T>>
where
    T: TryFrom<(String, String)>,
    // Accept every type of error for color_eyre
    <T as TryFrom<(String, String)>>::Error: std::error::Error + Send + Sync + 'static,
{
    let mut data = vec![];

    if !defaults.is_empty() {
        let mut data: Vec<T> = Vec::with_capacity(defaults.len());
        for default in defaults {
            data.push(T::try_from(default.clone().parse_tuple()?)?);
        }
        return Ok(data);
    }

    for i in 1..MAX_ROUNDS {
        let name: String = text_prompt(&format!("{prompt} {i} Name"), true, None)?;

        if name.is_empty() {
            break;
        }

        let version: String = text_prompt(&format!("{prompt} {i} Version"), true, None)?;
        data.push((name, version).try_into()?);
    }
    Ok(data)
}

#[cfg(test)]
mod tests {
    //! When using `cargo nextest` as a test runner, trying to display a dialogue
    //! panics, due to the way the tests are seperated from a TTY environment.
    //!
    //! In some tests this is used to assert, that dialogues are started
    //! when expected.
    //!
    //! However when using the default `cargo test` tests have access to
    //! `STDOUT` and as such are able to print dialogues/don't panic.
    //! In that case these tests are skipped by manually triggering the expected
    //! panic.

    use episko_lib::metadata::property::Property;

    use crate::cli::tests::skip_if_stdout;

    use super::*;

    #[test]
    #[should_panic(expected = "IO error: not a terminal")]
    fn test_directory_starts_prompt() {
        skip_if_stdout();

        let result = directory_prompt(None);
        result.unwrap();
    }

    #[test]
    fn test_directory_with_default() {
        let dir = Utf8PathBuf::from(".");

        let result = directory_prompt(Some(dir.clone()));

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result, dir);
    }

    #[test]
    #[should_panic(expected = "IO error: not a terminal")]
    fn test_title_starts_prompt() {
        skip_if_stdout();

        let result = title_prompt(None);
        result.unwrap();
    }

    #[test]
    fn test_title_with_default() {
        let title = "Title".to_string();

        let result = title_prompt(Some(title.clone()));

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result, title);
    }

    #[test]
    #[should_panic(expected = "IO error: not a terminal")]
    fn test_description_starts_prompt() {
        skip_if_stdout();

        let result = description_prompt(None);
        result.unwrap();
    }

    #[test]
    fn test_description_with_default() {
        let desc = Some("description".to_string());

        let result = description_prompt(desc.clone());

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result, desc);
    }

    #[test]
    #[should_panic(expected = "IO error: not a terminal")]
    fn test_categories_starts_prompt() {
        skip_if_stdout();

        let result = categories_prompt(&[]);
        result.unwrap();
    }

    #[test]
    fn test_categories_with_default() {
        let cats = vec!["test".to_string(), "cool".to_string()];

        let result = categories_prompt(&cats);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.len(), cats.len());
        for (i, category) in result.iter().enumerate() {
            assert_eq!(category, &Category::from_str(&cats[i]).unwrap());
        }
    }

    #[test]
    #[should_panic(expected = "IO error: not a terminal")]
    fn test_languages_starts_prompt() {
        skip_if_stdout();

        let result = languages_prompt(&[]);
        result.unwrap();
    }

    #[test]
    fn test_languages_with_default() {
        let langs = vec![
            ("rust".to_string(), "1.84.0".to_string()),
            ("go".to_string(), "1.22.0".to_string()),
            ("asm".to_string(), "0".to_string()),
            ("asm".to_string(), "0".to_string()),
        ];

        looping_with_version(&langs, languages_prompt);
    }

    #[test]
    #[should_panic(expected = "name cant be empty")]
    fn test_empty_language_should_fail() {
        let langs = vec![":1.0".to_string()];

        let result = languages_prompt(&langs);
        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "IO error: not a terminal")]
    fn test_build_system_starts_prompt() {
        skip_if_stdout();
        let result = languages_prompt(&[]);
        result.unwrap();
    }

    #[test]
    fn test_build_systems_with_default() {
        let bss = vec![
            ("Cargo".to_string(), String::new()),
            ("CMake".to_string(), "22".to_string()),
        ];

        looping_with_version(&bss, build_systems_prompt);
    }

    #[test]
    #[should_panic(expected = "IO error: not a terminal")]
    fn test_ide_starts_prompt() {
        skip_if_stdout();

        let result = ide_prompt(None);
        result.unwrap();
    }

    #[test]
    fn test_ide_with_default() {
        let ide_str = "neovim".to_string();
        let ide = Ide::from_str(ide_str.as_str()).unwrap();
        let result = ide_prompt(Some(ide_str));

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(result.is_some());
        let result = result.unwrap();

        assert_eq!(result, ide);
    }

    #[test]
    #[should_panic(expected = "IO error: not a terminal")]
    fn test_url_starts_prompt() {
        skip_if_stdout();

        let result = ide_prompt(None);
        result.unwrap();
    }

    #[test]
    fn test_url_with_default() {
        let url = Some("https://episko.de".to_string());

        let result = repository_url_prompt(url.clone());

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result, url);
    }

    #[test]
    #[should_panic(expected = "IO error: not a terminal")]
    fn text_prompt_starts() {
        skip_if_stdout();

        text_prompt::<String>("test", true, None).unwrap();
    }

    #[test]
    #[should_panic(expected = "IO error: not a terminal")]
    fn optional_prompt_starts() {
        skip_if_stdout();

        optional_text_prompt::<String>("test", None).unwrap();
    }

    #[test]
    #[should_panic(expected = "IO error: not a terminal")]
    fn looping_prompt_starts() {
        skip_if_stdout();
        looping_prompt_with_version::<Language>("test", &[]).unwrap();
    }

    #[test]
    fn looping_no_prompt_with_default() {
        looping_prompt_with_version::<Language>("test", &["rust:1.84".to_string()]).unwrap();
    }

    fn looping_with_version<T>(
        values: &[(String, String)],
        func: impl FnOnce(&[String]) -> Result<Vec<T>>,
    ) where
        T: Property,
    {
        let combined_values: Vec<String> = values
            .iter()
            .map(|(name, version)| format!("{name}:{version}"))
            .collect();
        let result = func(&combined_values);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result.len(), values.len());

        for (i, val) in values.iter().enumerate() {
            assert_eq!(result[i].name(), val.0);
            if val.1.is_empty() {
                assert_eq!(result[i].version(), None);
            } else {
                assert_eq!(result[i].version(), Some(val.1.as_str()));
            }
        }
    }
}
