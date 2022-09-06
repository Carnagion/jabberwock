use std::fs;
use std::path::Path;

use hatter::Args;
use hatter::Result;
use hatter::Value;

use crate::utils::macros;

/// The environment variable that stores the path of the templates directory.
///
/// Only available if the `templates` feature is enabled.
pub const TEMPLATES_DIR_VAR: &str = "templates";

/// Transpiles a specified `.hat` file and includes its HTML output into the current `.hat` file.
///
/// Only available if the `templates` feature is enabled.
pub fn include(args: Args) -> Result<Value>
{
    let hat = fs::read_to_string(Path::new(macros::require_env_string!(crate::INPUT_DIR_VAR, args.env)?.to_str())
        .join(macros::require_env_string!(TEMPLATES_DIR_VAR, args.env)?.to_str())
        .join(args.need_string(0)?)
        .with_extension("hat"))?;
    args.env.push_scope();
    if let Some(params) = args.get(1)
    {
        args.env.set("args", params);
    }
    let html = args.env.render(&hat);
    args.env.pop_scope();
    html.map(|rendered| Value::String(rendered.into()))
}