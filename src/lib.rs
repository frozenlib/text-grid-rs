//! A library to create formatted plain-text tables.
//!
//! See [`Grid`] (High Level API) or [`GridBuf`] (Low Level API) for details.

#![doc(html_root_url = "https://docs.rs/text-grid/0.1.0")]

mod cell;
mod grid;
mod grid_buf;

pub use self::cell::*;
pub use self::grid::*;
pub use self::grid_buf::*;
