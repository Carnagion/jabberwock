use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use glob;

use crate::Bytes;
use crate::Generator;
use crate::Operation;

/// An [Operation] that copies external files to the output directory.
pub struct AssetCopier {
    paths: HashMap<PathBuf, PathBuf>,
}

impl AssetCopier {
    /// Returns a new [AssetCopier].
    pub fn new() -> Self {
        AssetCopier {
            paths: HashMap::new(),
        }
    }

    /// Copies a file or directory from the specified [Path] to the specified [Path] relative to a [Generator]'s output directory.
    pub fn copy(&mut self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> &mut Self {
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
                        .ok_or_else(|| format!("Error converting glob pattern \"{}/**/*\" to string", from.display()))?)
                    .map_err(|error| format!("Error expanding glob pattern \"{}/**/*\": {}",from.display() , error))?
                    .filter(|result| result.as_ref().map_or(false, |entry| entry.is_file()))
                    .map(|result| result.map_or_else(|error| Err(format!("Error accessing file or directory: {}", error)), Bytes::read))
                    .collect::<Result<Vec<Bytes>, _>>()?;
                for mut bytes in data {
                    bytes.path = generator.source.join(to.join(bytes.path
                        .strip_prefix(&from)
                        .map_err(|error| format!("Error converting source path \"{}\" to destination path: {}", bytes.path.display(), error))?));
                    generator.data.push(bytes);
                }
            }
        }
        Ok(())
    }
}