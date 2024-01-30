use crate::cells_csv_writer::write_csv;
use crate::{grid_builder::*, Cells, CellsSchema, CellsSchemaExt, DefaultCellsSchema};
/// Generate a table using the columns defined by [`CellsFormatter`](crate::CellsFormatter).
///
/// # Examples
///
/// ```
/// use text_grid::*;
/// struct RowData {
///     a: u32,
///     b: u32,
/// }
/// impl Cells for RowData {
///     fn fmt(f: &mut CellsFormatter<Self>) {
///         f.column("a", |s| s.a);
///         f.column("b", |s| s.b);
///     }
/// }
///
/// let rows = [
///     RowData { a: 300, b: 1 },
///     RowData { a: 2, b: 200 },
/// ];
/// let g = to_grid(rows);
/// assert_eq!(format!("\n{g}"), r#"
///   a  |  b  |
/// -----|-----|
///  300 |   1 |
///    2 | 200 |
/// "#);
/// ```
pub fn to_grid(iter: impl IntoIterator<Item = impl Cells>) -> String {
    to_grid_with_schema(iter, DefaultCellsSchema::default())
}
pub fn to_grid_with_schema<T>(
    iter: impl IntoIterator<Item = T>,
    schema: impl CellsSchema<Source = T>,
) -> String {
    GridBuilder::from_iter_with_schema(iter, &schema).to_string()
}
pub fn to_grid_with_schema_ref<'a, T: 'a>(
    iter: impl IntoIterator<Item = &'a T>,
    schema: impl CellsSchema<Source = T>,
) -> String {
    GridBuilder::from_iter_with_schema(iter, &schema.map_ref()).to_string()
}

pub fn to_csv(iter: impl IntoIterator<Item = impl Cells>) -> String {
    to_csv_with_schema(iter, DefaultCellsSchema::default())
}
pub fn to_csv_with_schema<T>(
    iter: impl IntoIterator<Item = T>,
    schema: impl CellsSchema<Source = T>,
) -> String {
    let mut bytes = Vec::new();
    {
        let mut csv_writer = csv::Writer::from_writer(&mut bytes);
        write_csv(&mut csv_writer, iter, &schema, ".").unwrap();
        csv_writer.flush().unwrap();
    }
    String::from_utf8(bytes).unwrap()
}
pub fn to_csv_with_schema_ref<'a, T: 'a>(
    iter: impl IntoIterator<Item = &'a T>,
    schema: impl CellsSchema<Source = T>,
) -> String {
    to_csv_with_schema(iter, schema.map_ref())
}
