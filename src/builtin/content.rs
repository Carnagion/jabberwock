use std::path::Path;

use hatter::Args;
use hatter::Result;
use hatter::Value;

use markdown;

use crate::utils::macros;

/// The environment variable that stores the path of the markdown directory.
///
/// Only available if the `markdown` feature is enabled.
pub const MARKDOWN_DIR_VAR: &str = "markdown";

/// Transpiles a specified `.md` file and includes its HTML output into the current `.hat` file.
///
/// Only available if the `markdown` feature is enabled.
pub fn content(args: Args) -> Result<Value>
{
    markdown::file_to_html(&Path::new(macros::require_env_string!(crate::INPUT_DIR_VAR, args.env)?.to_str())
            .join(macros::require_env_string!(MARKDOWN_DIR_VAR, args.env)?.to_str())
            .join(args.need_string(0)?)
            .with_extension("md"))
        .map_or_else(|error| Err(macros::hatter_error!(RuntimeError, format!("Invalid markdown: {error}"))), |markdown| Ok(Value::String(markdown.into())))
}