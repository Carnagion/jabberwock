use std::str;

use crate::Generator;
use crate::Operation;

pub struct HatterTranspiler {
}

impl HatterTranspiler {
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
            let hat = str::from_utf8(&bytes.raw).map_err(|error| error.to_string())?;
            let html = generator.env.render(hat).map_err(|error| error.to_string())?;
            bytes.raw = html.into_bytes();
            bytes.path = bytes.path.with_extension("html");
        }
        Ok(())
    }
}