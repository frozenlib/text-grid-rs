#[macro_use]
mod syn_utils;

mod bound;
mod derive_cells;

#[doc = include_str!("../../docs/derive_cells.md")]
#[proc_macro_derive(Cells, attributes(cells))]
pub fn derive_cells(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn_utils::into_macro_output(derive_cells::build(input.into()))
}
