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
    let in_dir_val = macros::require_env_var!(crate::INPUT_DIR_VAR, args.env)?.to_owned();
    let template_dir_val = macros::require_env_var!(TEMPLATES_DIR_VAR, args.env)?.to_owned();
    match (in_dir_val, template_dir_val)
    {
        (Value::String(in_dir_path), Value::String(template_dir_path)) =>
        {
            let hat = fs::read_to_string(Path::new(in_dir_path.to_str())
                    .join(template_dir_path.to_str())
                    .join(args.need_string(0)?)
                    .with_extension("hat"))?;
            args.env.push_scope();
            let html = args.env.render(&hat);
            args.env.pop_scope();
            html.map(|rendered| Value::String(rendered.into()))
        },
        (in_dir_val, template_dir_val) => Err(macros::hatter_error!(RuntimeError, format!(r#"Expected strings in "{}" and "{}", got: {} and {}"#, crate::INPUT_DIR_VAR, TEMPLATES_DIR_VAR, in_dir_val.typename(), template_dir_val.typename()))),
    }
}