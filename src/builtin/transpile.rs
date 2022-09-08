use std::str;

use crate::Generator;
use crate::Operation;

/// An [Operation] that transpiles Hatter files into HTML files.
pub struct HatterTranspiler {
}

impl HatterTranspiler {
    /// Returns a new [HatterTranspiler].
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Operation for HatterTranspiler {
    fn apply(self, generator: &mut Generator) -> Result<(), String> {
        let data = generator.data
            .iter_mut()
            .filter(|bytes| bytes.path.extension().map_or(false, |extension| extension == "hat"));
        for bytes in data {
            let hat = str::from_utf8(&bytes.raw).map_err(|error| format!("Error converting file \"{}\" to UTF-8 string: {}", bytes.path.display(), error))?;
            let html = generator.env.render(hat).map_err(|error| format!("Error transpiling Hatter file \"{}\" to HTML: {}", bytes.path.display(), error))?;
            bytes.raw = html.into_bytes();
            bytes.path = bytes.path.with_extension("html");
        }
        Ok(())
    }
}