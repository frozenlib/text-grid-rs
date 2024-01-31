use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};

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
    rows: impl IntoIterator<Item = impl Borrow<T>>,
    schema: impl CellsSchema<Source = T>,
) -> String {
    GridBuilder::from_iter_with_schema(rows, &schema).to_string()
}

/// Generate csv using the columns defined by [`Cells`](crate::Cells).
pub fn to_csv(rows: impl IntoIterator<Item = impl Cells>) -> String {
    to_csv_with_schema(rows, DefaultCellsSchema::default())
}

/// Generate csv using the columns defined by [`CellsSchema`](crate::CellsSchema).
pub fn to_csv_with_schema<T>(
    rows: impl IntoIterator<Item = impl Borrow<T>>,
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

/// A builder used to create plain-text table.
#[deprecated = "use `to_grid`"]
pub struct Grid<T, S = DefaultCellsSchema<T>> {
    source: Vec<T>,
    schema: S,
}

#[allow(deprecated)]
impl<T: Cells> Default for Grid<T, DefaultCellsSchema<T>> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(deprecated)]
impl<T: Cells> Grid<T, DefaultCellsSchema<T>> {
    /// Create a new `Grid` with [`DefaultCellsSchema`] and prepare header rows.
    pub fn new() -> Self {
        Self::with_schema(DefaultCellsSchema::default())
    }
}

#[allow(deprecated)]
impl<T, S: CellsSchema<Source = T>> Grid<T, S> {
    /// Create a new `Grid` with specified schema and prepare header rows.
    pub fn with_schema(schema: S) -> Self {
        Grid {
            source: Vec::new(),
            schema,
        }
    }

    pub fn from_iter_with_schema(iter: impl IntoIterator<Item = T>, schema: S) -> Self {
        let mut g = Self::with_schema(schema);
        g.extend(iter);
        g
    }

    /// Append a row to the bottom of the grid.
    pub fn push(&mut self, item: T) {
        self.source.push(item);
    }

    pub fn to_csv(&self) -> String {
        let mut bytes = Vec::new();
        {
            let mut csv_writer = csv::Writer::from_writer(&mut bytes);
            write_csv(&mut csv_writer, &self.source, &self.schema.as_ref(), ".").unwrap();
            csv_writer.flush().unwrap();
        }
        String::from_utf8(bytes).unwrap()
    }

    fn build(&self) -> GridBuilder {
        GridBuilder::from_iter_with_schema(&self.source, &self.schema.as_ref())
    }
}

#[allow(deprecated)]
impl<T, S: CellsSchema<Source = T>> Extend<T> for Grid<T, S> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.source.extend(iter);
    }
}

#[allow(deprecated)]
impl<T: Cells> FromIterator<T> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut g = Self::new();
        g.extend(iter);
        g
    }
}

#[allow(deprecated)]
impl<T, S: CellsSchema<Source = T>> Display for Grid<T, S> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.build(), f)
    }
}

#[allow(deprecated)]
impl<T, S: CellsSchema<Source = T>> Debug for Grid<T, S> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(&self.build(), f)
    }
}
