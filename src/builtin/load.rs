use std::fs;
use std::path::Path;

use hatter::Args;
use hatter::OMap;
use hatter::Result;
use hatter::Value;
use hatter::value::List;
use hatter::value::Map;

use toml::Value as Toml;

use crate::utils::macros;

/// The environment variable that stores the path of the variables directory.
///
/// Only available if the `variables` feature is enabled.
pub const VARIABLES_DIR_VAR: &str = "variables";

/// Returns a specified TOML file's contents transpiled into a Hatter [Value].
///
/// Only available if the `variables` feature is enabled.
pub fn load(args: Args) -> Result<Value>
{
    let in_dir_val = macros::require_env_string!(crate::INPUT_DIR_VAR, args.env)?;
    let vars_dir_val = macros::require_env_string!(VARIABLES_DIR_VAR, args.env)?;
    Ok(Value::Map(Map::new(fs::read_to_string(Path::new(in_dir_val.to_str())
        .join(vars_dir_val.to_str())
        .join(args.need_string(0)?)
        .with_extension("toml"))?
        .parse::<Toml>()
        .map_err(|error| macros::hatter_error!(RuntimeError, format!("Invalid TOML: {error}")))?
        .as_table()
        .ok_or_else(|| macros::hatter_error!(RuntimeError, "Expected TOML table at top level"))?
        .iter()
        .fold(OMap::new(), |mut map, (key, val)|
        {
            map.insert(key, toml_to_value(val));
            map
        }))))
}

fn toml_to_value(toml: &Toml) -> Value
{
    match toml
    {
        Toml::Boolean(bool) => Value::Bool(*bool),
        Toml::Integer(int) => Value::Number(*int as f64),
        Toml::Float(float) => Value::Number(*float),
        Toml::String(str) => Value::String(str.into()),
        Toml::Array(array) => Value::List(List::new(array.into_iter()
            .map(toml_to_value)
            .collect())),
        Toml::Table(table) => Value::Map(Map::new(table.into_iter()
            .map(|(key, val)| (key, toml_to_value(val)))
            .fold(OMap::new(), |mut map, (key, val)|
            {
                map.insert(key, val);
                map
            }))),
        Toml::Datetime(date) => Value::String(date.to_string().into()),
    }
}