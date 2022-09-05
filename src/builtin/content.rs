use std::path::Path;

use hatter::Args;
use hatter::Result;
use hatter::Value;

use markdown;

use crate::utils::macros;

/// The environment variable that stores the path of the markdown directory.
pub const MARKDOWN_DIR_VAR: &str = "markdown";

/// Transpiles a specified `.md` file and includes its HTML output into the current `.hat` file.
pub fn content(args: Args) -> Result<Value>
{
    let in_dir_val = macros::require_env_var!(crate::INPUT_DIR_VAR, args.env)?.to_owned();
    let markdown_dir_val = macros::require_env_var!(MARKDOWN_DIR_VAR, args.env)?.to_owned();
    match (in_dir_val, markdown_dir_val)
    {
        (Value::String(in_dir_path), Value::String(markdown_dir_path)) => markdown::file_to_html(&Path::new(in_dir_path.to_str())
                .join(markdown_dir_path.to_str())
                .join(args.need_string(0)?)
                .with_extension("md"))
            .map_or_else(|error| Err(macros::hatter_error!(RuntimeError, format!("Could not parse markdown: {error}"))), |markdown| Ok(Value::String(markdown.into()))),
        (in_dir_val, markdown_dir_val) => Err(macros::hatter_error!(RuntimeError, format!(r#"Expected strings in "{}" and "{}", got: {} and {}"#, crate::INPUT_DIR_VAR, MARKDOWN_DIR_VAR, in_dir_val.typename(), markdown_dir_val.typename()))),
    }
}