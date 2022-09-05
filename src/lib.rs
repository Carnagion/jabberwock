//! A simple, yet extendable static site generator (SSG) using [Hatter](https://docs.rs/hatter/latest/hatter/) as its templating language.
//!
//! Raven transpiles `.hat` files from an input directory into `.html` files in an output directory, and can be easily extended by adding file rules and functions through Hatter's API.
//! Some minimal specialized features such as templates, loading variables, and transpiling markdown are included by default.

#[warn(missing_docs)]

pub mod builtin;

mod prelude;
pub use prelude::*;

mod utils;