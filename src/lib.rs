//! A simple, yet highly modular static site generator using [Hatter](https://github.com/xvxx/hatter) as its templating language.

#[warn(missing_docs)]

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use glob;

use hatter::Env;

#[cfg(any(feature = "copy", feature = "markdown", feature = "templates", feature = "toml", feature = "transpile"))]
pub mod builtin;

mod macros;

/// A sequence of bytes originating from a file path.
pub struct Bytes {
    /// Origin of the byte content. It may not actually exist, but it must evaluate to a descendant of a [Generator]'s input directory if used in it.
    pub path: PathBuf,
    /// Raw byte content of a file.
    pub raw: Vec<u8>,
}

impl Bytes {
    /// Reads raw byte content from the file specified by the [Path] and returns it as [Bytes].
    ///
    /// # Errors
    /// See [fs::read()].
    pub fn read(path: impl AsRef<Path>) -> Result<Bytes, String> {
        let path_ref = path.as_ref();
        Ok(Bytes {
            path: path_ref.into(),
            raw: fs::read(path_ref)
                .map_err(|error| format!("Error reading data at \"{}\": {}", path_ref.display(), error))?,
        })
    }
}

/// An operation that can be applied to the data in a generator.
pub trait Operation {
    /// Applies a modification to the data in the specified [Generator].
    ///
    /// # Errors
    /// Returns an error if the operation could not be applied or the data was invalid.
    fn apply(self, generator: &mut Generator) -> Result<(), String>;
}

/// A site generator that can read byte contents from an input directory, perform operations on them, and write the results to an output directory.
pub struct Generator {
    /// Top-level Hatter [Env].
    pub env: Env,
    /// Raw byte content of each file in the input directory, represented as [Bytes].
    pub data: Vec<Bytes>,
    source: PathBuf,
}

impl Generator {
    /// Returns a new [Generator] containing data from the input directory specified by the [Path].
    ///
    /// # Errors
    /// Returns an error in many cases, including but not limited to:
    /// - The path does not exist or could not be accessed
    /// - The path contains non-UTF-8 characters
    /// - A descendant of the path could not be accessed
    pub fn source(path: impl AsRef<Path>) -> Result<Self, String> {
        let path_ref = path.as_ref();
        Ok(Generator {
            env: Env::new(),
            data: glob::glob(path_ref
                        .join("**")
                        .join("*")
                        .to_str()
                        .ok_or_else(|| format!("Error converting glob pattern \"{}/**/*\" to string", path_ref.display()))?)
                    .map_err(|error| format!("Error expanding glob pattern \"{}/**/*\": {}",path_ref.display() , error))?
                    .filter(|result| result.as_ref().map_or(false, |entry| entry.is_file()))
                    .map(|result| result.map_or_else(|error| Err(format!("Error accessing file or directory: {}", error)), Bytes::read))
                    .collect::<Result<Vec<_>, _>>()?,
            source: path_ref.into(),
        })
    }

    /// Applies the specified [Operation] to the data.
    ///
    /// # Errors
    /// See [Operation::apply()].
    pub fn apply(&mut self, operation: impl Operation) -> Result<&mut Self, String> {
        operation.apply(self)?;
        Ok(self)
    }

    /// Writes the resulting data to the output directory specified by the [Path].
    ///
    /// # Errors
    /// Returns an error in many cases, including but not limited to:
    /// - The output directory could not be accessed, cleared, or created
    /// - One or more of the input file paths could not be converted to an output path relative to the output directory
    /// - One or more of the output files or their parent directories could not be created or written to
    pub fn destination(self, path: impl AsRef<Path>) -> Result<(), String> {
        let path_ref = path.as_ref();
        fs::remove_dir_all(path_ref).map_err(|error| format!("Error removing directory \"{}\" and its contents: {}", path_ref.display(), error))?;
        fs::create_dir_all(path_ref).map_err(|error| format!("Error creating directory \"{}\": {}", path_ref.display(), error))?;
        for bytes in self.data {
            let destination = path_ref.join(bytes.path
                .strip_prefix(&self.source)
                .map_err(|error| format!("Error converting source path \"{}\" to destination path: {}", bytes.path.display(), error))?);
            fs::create_dir_all(destination.parent()
                    .ok_or_else(|| format!("Error finding parent of \"{}\"", destination.display()))?)
                .map_err(|error| format!("Error creating parent of \"{}\": {}", destination.display(), error))?;
            fs::write(&destination, bytes.raw).map_err(|error| format!("Error writing data to \"{}\": {}", destination.display(), error))?;
        }
        Ok(())
    }
}