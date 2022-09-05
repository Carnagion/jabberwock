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
pub const VARIABLES_DIR_VAR: &str = "variables";

/// Loads variables defined in a specified `.toml` file into the current Hatter environment.
pub fn load(args: Args) -> Result<Value>
{
    let in_dir_val = macros::require_env_var!(crate::INPUT_DIR_VAR, args.env)?.to_owned();
    let toml_dir_val = macros::require_env_var!(VARIABLES_DIR_VAR, args.env)?.to_owned();
    match (in_dir_val, toml_dir_val)
    {
        (Value::String(in_dir_path), Value::String(toml_dir_path)) =>
        {
            fs::read_to_string(Path::new(in_dir_path.to_str())
                    .join(toml_dir_path.to_str())
                    .join(args.need_string(0)?)
                    .with_extension("toml"))?
                .parse::<Toml>()
                .map_err(|error| macros::hatter_error!(RuntimeError, format!("Invalid TOML: {error}")))?
                .as_table()
                .ok_or_else(|| macros::hatter_error!(RuntimeError, "Invalid TOML"))?
                .iter()
                .for_each(|(key, val)| args.env.set(key, toml_to_value(val)));
            Ok(Value::None)
        },
        (in_dir_val, toml_dir_val) => Err(macros::hatter_error!(RuntimeError, format!(r#"Expected strings in "{}" and "{}", got: {} and {}"#, crate::INPUT_DIR_VAR, VARIABLES_DIR_VAR, in_dir_val.typename(), toml_dir_val.typename()))),
    }
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