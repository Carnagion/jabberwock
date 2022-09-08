use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use glob;

use crate::Bytes;
use crate::Generator;
use crate::Operation;

pub struct AssetCopier {
    paths: HashMap<PathBuf, PathBuf>,
}

impl AssetCopier {
    pub fn new() -> Self {
        AssetCopier {
            paths: HashMap::new(),
        }
    }

    pub fn include(&mut self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> &mut Self {
        self.paths.insert(from.as_ref().into(), to.as_ref().into());
        self
    }
}

impl Operation for AssetCopier {
    fn apply(self, generator: &mut Generator) -> Result<(), String> {
        for (from, to) in self.paths {
            let projection = generator.source.join(&to);
            if from.is_file() {
                let mut bytes = Bytes::read(from)?;
                bytes.path = projection;
                generator.data.push(bytes);
            }
            else {
                let data = glob::glob(&from
                        .join("**")
                        .join("*")
                        .to_str()
                        .ok_or_else(|| String::from(""))?)
                    .map_err(|error| error.to_string())?
                    .filter(|result| result.as_ref().map_or(false, |entry| entry.is_file()))
                    .map(|result| result.map_or_else(|error| Err(error.to_string()), Bytes::read))
                    .collect::<Result<Vec<Bytes>, _>>()?;
                for mut bytes in data {
                    bytes.path = generator.source.join(to.join(bytes.path
                        .strip_prefix(&from)
                        .map_err(|error| error.to_string())?));
                    generator.data.push(bytes);
                }
            }
        }
        Ok(())
    }
}