//! A library to create formatted plain-text tables.
//!
//! See [`Grid`] (High Level API) or [`GridBuilder`] (Low Level API) for details.

mod cell;
mod cells;
mod grid;
mod grid_builder;

pub use self::cell::*;
pub use self::cells::*;
pub use self::grid::*;
pub use self::grid_builder::*;

#[cfg(doctest)]
mod tests {
    #[doc = include_str!("../README.md")]
    mod readme {}
}
