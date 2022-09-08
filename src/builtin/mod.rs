//! Contains built-in operations and functions to assist in generating a static site.
//!
//! Each feature is enabled by default, but can optionally be disabled.

#[cfg(feature = "hatter")]
mod transpile;
#[cfg(feature = "hatter")]
pub use transpile::*;

#[cfg(feature = "markdown")]
mod content;
#[cfg(feature = "markdown")]
pub use content::*;

#[cfg(feature = "templates")]
mod include;
#[cfg(feature = "templates")]
pub use include::*;

#[cfg(feature = "toml")]
mod load;
#[cfg(feature = "toml")]
pub use load::*;

#[cfg(feature = "copy")]
mod copy;
#[cfg(feature = "copy")]
pub use copy::*;