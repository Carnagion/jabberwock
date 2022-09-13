use std::fs;
use std::path::Path;

use hatter;
use hatter::Args;
use hatter::OMap;
use hatter::Value;
use hatter::value::List;
use hatter::value::Map;

use toml::Value as Toml;

use crate::Generator;
use crate::Operation;
use crate::macros;

const VARIABLES_DIR_VAR: &str = "variables";

/// An [Operation] that adds a Hatter function to transpile TOML files into Hatter values.
pub struct TomlTranspiler {
    source: String,
}

impl TomlTranspiler {
    /// Returns a new [TomlTranspiler] that looks for TOML files in the directory specified by the [Path].
    pub fn source(path: impl Into<String>) -> Self {
        Self {
            source: path.into(),
        }
    }
}

impl Operation for TomlTranspiler {
    fn apply(self, generator: &mut Generator) -> Result<(), String> {
        generator.env.set(VARIABLES_DIR_VAR, self.source);
        generator.env.set("load", load);
        Ok(())
    }
}

fn load(args: Args) -> hatter::Result<Value> {
    Ok(Value::Map(Map::new(fs::read_to_string(Path::new(macros::require_env_string!(VARIABLES_DIR_VAR, args.env)?.to_str())
        .join(args.need_string(0)?)
        .with_extension("toml"))?
        .parse::<Toml>()
        .map_err(|error| macros::error!(RuntimeError, error.to_string()))?
        .as_table()
        .ok_or_else(|| macros::error!(RuntimeError, "Expected table at top level of TOML"))?
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