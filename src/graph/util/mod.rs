#[macro_use]
pub(crate) mod macros;

pub(crate) mod parser;
mod paths;

pub use paths::{BreadthFirstPaths, DepthFirstPaths, Paths};
