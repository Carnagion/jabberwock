use std::collections::HashMap;
use std::fs;
use std::path::Path;

use glob;

use hatter::Result;

use crate::utils::macros;

mod config;
pub use config::*;

/// The environment variable that stores the path of the input directory.
pub const INPUT_DIR_VAR: &str = "input";

/// The environment variable that stores the path of the output directory.
pub const OUTPUT_DIR_VAR: &str = "output";

/// Builds a static website by transpiling input Hatter files to output HTML files using the default [Config].
///
/// This is the same as `raven::build_with(&mut Config::default())`.
///
/// # Errors
///
/// See [build_with()].
///
/// # Examples
/// ```rust
/// use raven;
///
/// let result = raven::build();
/// println!("{:?}", result);
/// ```
pub fn build() -> Result<()>
{
    build_with(&mut Config::default())
}

/// Builds a static website by transpiling input Hatter files to output HTML files as specified by a [Config].
///
/// # Errors
///
/// Returns an error in the following situations, but is not limited to these:
/// - A file or directory could not be accessed in the input directory
/// - A file or directory could not be created in the output directory
/// - Hatter was unable to transpile an input file's contents
///
/// # Examples
/// ```rust
/// use raven;
/// use raven::Config;
///
/// let mut config = Config::default();
/// let result = raven::build_with(&mut config);
///
/// println!("{:?}", result);
/// ```
pub fn build_with(config: &mut Config) -> Result<()>
{
    let in_dir_path = Path::new(&config.input_dir);
    let out_dir_path = Path::new(&config.output_dir);

    fs::remove_dir_all(out_dir_path)?;
    fs::create_dir_all(out_dir_path)?;

    let mut pattern_rules = HashMap::new();
    for (pattern, rule) in config.file_rules()
    {
        let paths = glob::glob(in_dir_path.join(pattern)
                .to_str()
                .ok_or_else(|| macros::hatter_error!(RuntimeError, "Input directory path or glob pattern contains invalid UTF-8 characters"))?)
            .map_err(|error| macros::hatter_error!(RuntimeError, format!("Invalid glob pattern: {error}")))?;
        for result in paths
        {
            pattern_rules.insert(result.map_err(|error| macros::hatter_error!(RuntimeError, format!("Inaccessible input file or directory: {error}")))?, *rule);
        }
    }

    let results = glob::glob(in_dir_path.join("**/*")
            .to_str()
            .ok_or_else(|| macros::hatter_error!(RuntimeError, "Input directory path or glob pattern contains invalid UTF-8 characters"))?)
        .map_err(|error| macros::hatter_error!(RuntimeError, format!("Invalid glob pattern: {error}")))?;
    for result in results
    {
        let in_path = result.map_err(|error| macros::hatter_error!(RuntimeError, format!("Inaccessible input file or directory: {error}")))?;
        if in_path.is_dir()
        {
            continue;
        }
        let out_path = out_dir_path.join(in_path
            .strip_prefix(in_dir_path)
            .map_err(|error| macros::hatter_error!(RuntimeError, format!("Expected input file to be descendant of input directory: {error}")))?);
        match pattern_rules.get(&in_path).unwrap_or(&FileRule::Ignore)
        {
            FileRule::Ignore => continue,
            FileRule::Copy =>
            {
                fs::create_dir_all(out_path.parent().ok_or_else(|| macros::hatter_error!(RuntimeError, format!("Error creating directory: {}", in_path.display())))?)?;
                fs::copy(in_path, out_path)?;
            },
            FileRule::Transpile =>
            {
                let hat = fs::read_to_string(&in_path)?;
                let html = config.env.render(&hat)?;
                fs::create_dir_all(out_path.parent().ok_or_else(|| macros::hatter_error!(RuntimeError, format!("Error creating directory: {}", in_path.display())))?)?;
                fs::write(out_path.with_extension("html"), html)?;
            },
        }
    }

    Ok(())
}