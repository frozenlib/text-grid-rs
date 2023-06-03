use crate::grid_builder::*;
use std::fmt::*;
use std::marker::PhantomData;
/// A builder used to create plain-text table from row values.
///
/// Generate a table using the columns defined by [`CellsFormatter`].
///
/// # Examples
///
/// ```
/// use text_grid::*;
/// struct RowData {
///     a: u32,
///     b: u32,
/// }
/// impl CellsSource for RowData {
///     fn fmt(f: &mut CellsFormatter<&Self>) {
///         f.column("a", |s| s.a);
///         f.column("b", |s| s.b);
///     }
/// }
///
/// let mut g = Grid::new();
/// g.push(&RowData { a: 300, b: 1 });
/// g.push(&RowData { a: 2, b: 200 });
///
/// assert_eq!(format!("\n{g}"), r#"
///   a  |  b  |
/// -----|-----|
///  300 |   1 |
///    2 | 200 |
/// "#);
/// ```
pub struct Grid<R: ?Sized, S> {
    b: GridBuilder,
    schema: S,
    _phantom: PhantomData<fn(&R)>,
}

impl<R: CellsSource + ?Sized> Default for Grid<R, DefaultGridSchema<R>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: CellsSource + ?Sized> Grid<R, DefaultGridSchema<R>> {
    /// Create a new `Grid` with [`DefaultGridSchema`] and prepare header rows.
    pub fn new() -> Self {
        Self::new_with_schema(DefaultGridSchema::default())
    }
}

impl<R: ?Sized, S: GridSchema<R>> Grid<R, S> {
    /// Create a new `Grid` with specified schema and prepare header rows.
    pub fn new_with_schema(schema: S) -> Self {
        let b = GridBuilder::new_with_header(&schema);
        Grid {
            b,
            schema,
            _phantom: PhantomData::default(),
        }
    }
}
impl<R: ?Sized, S: GridSchema<R>> Grid<R, S> {
    /// Append a row to the bottom of the grid.
    pub fn push(&mut self, source: &R) {
        self.b.push(|b| b.extend_with_schema(source, &self.schema));
    }

    /// Append a row separator to the bottom of the grid.
    pub fn push_separator(&mut self) {
        self.b.push_separator();
    }
}
impl<A: AsRef<R>, R, S: GridSchema<R>> Extend<A> for Grid<R, S> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        for i in iter {
            self.push(i.as_ref());
        }
    }
}

impl<R: ?Sized, S> Display for Grid<R, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&self.b, f)
    }
}
impl<R: ?Sized, S> Debug for Grid<R, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&self.b, f)
    }
}
