//! A collection of useful built-in Hatter functions.

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