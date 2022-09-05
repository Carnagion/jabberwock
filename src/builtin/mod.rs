//! A collection of useful built-in Hatter functions.
//!
//! Only available if any of the `markdown`, `templates`, or `variables` features are enabled.

#[cfg(feature = "markdown")]
mod content;
#[cfg(feature = "markdown")]
pub use content::*;

#[cfg(feature = "templates")]
mod include;
#[cfg(feature = "templates")]
pub use include::*;

#[cfg(feature = "variables")]
mod load;
#[cfg(feature = "variables")]
pub use load::*;