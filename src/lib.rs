use std::fs;
use std::path::Path;
use std::path::PathBuf;

use glob;

use hatter::Env;

#[cfg(any(feature = "copy", feature = "markdown", feature = "templates", feature = "toml", feature = "transpile"))]
pub mod builtin;

mod macros;

pub struct Bytes {
    pub path: PathBuf,
    pub raw: Vec<u8>,
}

impl Bytes {
    pub fn read(path: impl AsRef<Path>) -> Result<Bytes, String> {
        let path_ref = path.as_ref();
        Ok(Bytes {
            path: path_ref.into(),
            raw: fs::read(path_ref)
                .map_err(|error| error.to_string())?,
        })
    }
}

pub trait Operation {
    fn apply(self, generator: &mut Generator) -> Result<(), String>;
}

pub struct Generator {
    pub env: Env,
    pub data: Vec<Bytes>,
    source: PathBuf,
}

impl Generator {
    pub fn source(path: impl AsRef<Path>) -> Result<Self, String> {
        let path_ref = path.as_ref();
        Ok(Generator {
            env: Env::new(),
            data: glob::glob(path_ref
                        .join("**")
                        .join("*")
                        .to_str()
                        .ok_or_else(|| String::from(""))?)
                    .map_err(|error| error.to_string())?
                    .filter(|result| result.as_ref().map_or(false, |entry| entry.is_file()))
                    .map(|result| result.map_or_else(|error| Err(error.to_string()), Bytes::read))
                    .collect::<Result<Vec<_>, _>>()?,
            source: path_ref.into(),
        })
    }

    pub fn apply(&mut self, operation: impl Operation) -> Result<&mut Self, String> {
        operation.apply(self)?;
        Ok(self)
    }

    pub fn destination(self, path: impl AsRef<Path>) -> Result<(), String> {
        let path_ref = path.as_ref();
        fs::remove_dir_all(path_ref).map_err(|error| error.to_string())?;
        fs::create_dir_all(path_ref).map_err(|error| error.to_string())?;
        for bytes in self.data {
            let destination = path_ref.join(bytes.path
                .strip_prefix(&self.source)
                .map_err(|error| error.to_string())?);
            fs::create_dir_all(destination.parent()
                    .ok_or_else(|| String::from(""))?)
                .map_err(|error| error.to_string())?;
            fs::write(destination, bytes.raw).map_err(|error| error.to_string())?;
        }
        Ok(())
    }
}