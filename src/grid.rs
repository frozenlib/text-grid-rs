use crate::cells_csv_writer::write_csv;
use crate::{grid_builder::*, Cells, CellsSchema, CellsSchemaExt, DefaultCellsSchema};
use std::fmt::*;
/// A builder used to create plain-text table from row values.
///
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
/// let mut g = Grid::new();
/// g.push(RowData { a: 300, b: 1 });
/// g.push(RowData { a: 2, b: 200 });
///
/// assert_eq!(format!("\n{g}"), r#"
///   a  |  b  |
/// -----|-----|
///  300 |   1 |
///    2 | 200 |
/// "#);
/// ```
pub struct Grid<T, S = DefaultCellsSchema<T>> {
    source: Vec<T>,
    schema: S,
}

impl<T: Cells> Default for Grid<T, DefaultCellsSchema<T>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Cells> Grid<T, DefaultCellsSchema<T>> {
    /// Create a new `Grid` with [`DefaultCellsSchema`] and prepare header rows.
    pub fn new() -> Self {
        Self::with_schema(DefaultCellsSchema::default())
    }
}

impl<T, S: CellsSchema<Source = T>> Grid<S::Source, S> {
    /// Create a new `Grid` with specified schema and prepare header rows.
    pub fn with_schema(schema: S) -> Self {
        Grid {
            source: Vec::new(),
            schema,
        }
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
        let mut b = GridBuilder::new_with_header(&self.schema);
        for source in &self.source {
            b.push_body_with_schema(source, &self.schema);
        }
        b
    }
}
impl<T, S: CellsSchema<Source = T>> Grid<T, S> {
    /// Append a row to the bottom of the grid.
    pub fn push(&mut self, item: T) {
        self.source.push(item);
    }
}
impl<T, S: CellsSchema<Source = T>> Extend<T> for Grid<T, S> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.source.extend(iter);
    }
}
impl<T: Cells> FromIterator<T> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut g = Self::new();
        g.extend(iter);
        g
    }
}

impl<T, S: CellsSchema<Source = T>> Display for Grid<T, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&self.build(), f)
    }
}
impl<T, S: CellsSchema<Source = T>> Debug for Grid<T, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&self.build(), f)
    }
}
