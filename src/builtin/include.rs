use std::fs;
use std::path::Path;

use hatter;
use hatter::Args;
use hatter::Value;

use crate::Generator;
use crate::Operation;
use crate::macros;

const TEMPLATES_DIR_VAR: &str = "templates";

/// An [Operation] that adds a Hatter function to transpile and include the contents of a Hatter file in another Hatter file.
pub struct TemplateTranspiler {
    source: String,
}

impl TemplateTranspiler {
    /// Returns a new [TemplateTranspiler] that looks for Hatter template files in the directory specified by the [Path].
    pub fn source(path: impl Into<String>) -> Self {
        Self {
            source: path.into(),
        }
    }
}

impl Operation for TemplateTranspiler {
    fn apply(self, generator: &mut Generator) -> Result<(), String> {
        generator.env.set(TEMPLATES_DIR_VAR, self.source);
        generator.env.set("include", include);
        Ok(())
    }
}

fn include(args: Args) -> hatter::Result<Value> {
    let hat = fs::read_to_string(Path::new(macros::require_env_string!(TEMPLATES_DIR_VAR, args.env)?.to_str())
        .join(args.need_string(0)?)
        .with_extension("hat"))?;
    args.env.push_scope();
    if let Some(params) = args.get(1) {
        args.env.set("args", params);
    }
    let html = args.env.render(&hat);
    args.env.pop_scope();
    html.map(|rendered| Value::String(rendered.into()))
}