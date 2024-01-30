use crate::cells_csv_writer::write_csv;
use crate::{grid_builder::*, Cells, CellsSchema, CellsSchemaExt, DefaultCellsSchema};
/// Generate a table using the columns defined by [`Cells`](crate::Cells).
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
pub fn to_grid(rows: impl IntoIterator<Item = impl Cells>) -> String {
    to_grid_with_schema(rows, DefaultCellsSchema::default())
}

/// Generate a table using the columns defined by [`CellsSchema`](crate::CellsSchema).
pub fn to_grid_with_schema<T>(
    rows: impl IntoIterator<Item = T>,
    schema: impl CellsSchema<Source = T>,
) -> String {
    GridBuilder::from_iter_with_schema(rows, &schema).to_string()
}

/// Generate a table using the columns defined by [`CellsSchema`](crate::CellsSchema).
pub fn to_grid_with_schema_ref<'a, T: 'a>(
    rows: impl IntoIterator<Item = &'a T>,
    schema: impl CellsSchema<Source = T>,
) -> String {
    GridBuilder::from_iter_with_schema(rows, &schema.map_ref()).to_string()
}

/// Generate csv using the columns defined by [`Cells`](crate::Cells).
pub fn to_csv(rows: impl IntoIterator<Item = impl Cells>) -> String {
    to_csv_with_schema(rows, DefaultCellsSchema::default())
}

/// Generate csv using the columns defined by [`CellsSchema`](crate::CellsSchema).
pub fn to_csv_with_schema<T>(
    rows: impl IntoIterator<Item = T>,
    schema: impl CellsSchema<Source = T>,
) -> String {
    let mut bytes = Vec::new();
    {
        let mut csv_writer = csv::Writer::from_writer(&mut bytes);
        write_csv(&mut csv_writer, rows, &schema, ".").unwrap();
        csv_writer.flush().unwrap();
    }
    String::from_utf8(bytes).unwrap()
}

/// Generate csv using the columns defined by [`CellsSchema`](crate::CellsSchema).
pub fn to_csv_with_schema_ref<'a, T: 'a>(
    rows: impl IntoIterator<Item = &'a T>,
    schema: impl CellsSchema<Source = T>,
) -> String {
    to_csv_with_schema(rows, schema.map_ref())
}
