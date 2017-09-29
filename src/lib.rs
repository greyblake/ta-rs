#[macro_use]
extern crate error_chain;

#[cfg(test)]
mod test_helper;

pub mod indicators;
pub mod errors;

mod traits;
pub use traits::*;

