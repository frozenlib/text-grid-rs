//! A library to create formatted plain-text tables.
//!
//! See [`to_grid`] (High Level API) or [`GridBuilder`] (Low Level API) for details.

mod cell;
mod cells;
mod cells_csv_writer;
mod cells_formatter;
mod grid;
mod grid_builder;

pub use self::cell::*;
pub use self::cells::*;
pub use self::cells_formatter::*;
pub use self::grid::*;
pub use self::grid_builder::*;

#[cfg(doctest)]
mod tests {
    #[doc = include_str!("../README.md")]
    mod readme {}
}
