use std::path::Path;

use hatter::Args;
use hatter::Value;

use markdown;

use crate::Generator;
use crate::Operation;
use crate::macros;

const MARKDOWN_DIR_VAR: &str = "markdown";

/// An [Operation] that adds a Hatter function to transpile Markdown to HTML.
pub struct MarkdownTranspiler {
    source: String,
}

impl MarkdownTranspiler {
    /// Returns a new [MarkdownTranspiler] that looks for Markdown files in the directory specified by the [Path].
    pub fn source(path: impl Into<String>) -> Self {
        Self {
            source: path.into(),
        }
    }
}

impl Operation for MarkdownTranspiler {
    fn apply(self, generator: &mut Generator) -> Result<(), String> {
        generator.env.set(MARKDOWN_DIR_VAR, self.source);
        generator.env.set("content", content);
        Ok(())
    }
}

fn content(args: Args) -> hatter::Result<Value> {
    markdown::file_to_html(&Path::new(macros::require_env_string!(MARKDOWN_DIR_VAR, args.env)?.to_str())
            .join(args.need_string(0)?)
            .with_extension("md"))
        .map_or_else(|error| Err(macros::error!(RuntimeError, format!("Error parsing Markdown file to HTML: {}", error))), |markdown| Ok(Value::String(markdown.into())))
}