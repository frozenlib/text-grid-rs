//! A library to create formatted plain-text tables.
//!
//! See [`Grid`] (High Level API) or [`GridBuilder`] (Low Level API) for details.
#![doc(html_root_url = "https://docs.rs/text-grid/0.2.0")]

mod cell;
mod grid;
mod grid_builder;

pub use self::cell::*;
pub use self::grid::*;
pub use self::grid_builder::*;

#[cfg(doctest)]
mod tests {
    #[doc = include_str!("../README.md")]
    mod readme {}
}
