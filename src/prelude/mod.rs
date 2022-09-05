use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use glob;

use hatter::Error;

use crate::utils::macros;

mod config;
pub use config::*;

pub const INPUT_DIR_VAR: &str = "input";
pub const OUTPUT_DIR_VAR: &str = "output";

pub fn build(config: &mut Config) -> Result<(), Error>
{
    let file_rules = self::build_file_rules(config)?;
    config.env.push_scope();
    config.env.set(INPUT_DIR_VAR, config.get_input_dir());
    config.env.set(OUTPUT_DIR_VAR, config.get_output_dir());
    let build_result = self::build_all_files(config, &file_rules);
    config.env.pop_scope();
    build_result
}

fn build_file_rules(config: &mut Config) -> Result<HashMap<PathBuf, FileRule>, Error>
{
    let mut file_rules = HashMap::new();
    for (pattern, rule) in config.file_rules()
    {
        let paths = glob::glob(pattern.as_str())
            .map_err(|error| macros::hatter_error!(RuntimeError, format!("{error}")))?;
        for result in paths
        {
            file_rules.insert(result.map_err(|error| macros::hatter_error!(RuntimeError, format!("{error}")))?, *rule);
        }
    }
    Ok(file_rules)
}

fn build_all_files(config: &mut Config, file_rules: &HashMap<PathBuf, FileRule>) -> Result<(), Error>
{
    for (in_path, rule) in file_rules.iter().filter(|(path, _)| path.is_file())
    {
        match rule
        {
            FileRule::Ignore => continue,
            FileRule::Copy =>
            {
                let out_path = Path::new(config.get_output_dir()).join(in_path
                    .strip_prefix(Path::new(config.get_input_dir()))
                    .map_err(|error| macros::hatter_error!(RuntimeError, format!("{error}")))?);
                fs::create_dir_all(out_path.parent().ok_or_else(|| macros::hatter_error!(RuntimeError, ""))?)?;
                fs::copy(in_path, out_path)?;
            },
            FileRule::Transpile =>
            {
                let out_path = Path::new(config.get_output_dir()).join(in_path
                    .strip_prefix(Path::new(config.get_input_dir()))
                    .map_err(|error| macros::hatter_error!(RuntimeError, format!("{error}")))?);
                let hat = fs::read_to_string(in_path)?;
                let html = config.env.render(&hat)?;
                fs::create_dir_all(out_path.parent().ok_or_else(|| macros::hatter_error!(RuntimeError, ""))?)?;
                fs::write(out_path.with_extension("html"), html)?;
            }
        }
    }
    Ok(())
}