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
/// impl Cells for RowData {
///     fn fmt(f: &mut CellsFormatter<Self>) {
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
pub struct Grid<R: ?Sized, S = DefaultCellsSchema<R>> {
    b: GridBuilder,
    schema: S,
    _phantom: PhantomData<fn(&R)>,
}

impl<R: Cells + ?Sized> Default for Grid<R, DefaultCellsSchema<R>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Cells + ?Sized> Grid<R, DefaultCellsSchema<R>> {
    /// Create a new `Grid` with [`DefaultCellsSchema`] and prepare header rows.
    pub fn new() -> Self {
        Self::new_with_schema(DefaultCellsSchema::default())
    }
}

impl<R: ?Sized, S: CellsSchema<Source = R>> Grid<S::Source, S> {
    /// Create a new `Grid` with specified schema and prepare header rows.
    pub fn new_with_schema(schema: S) -> Self {
        let b = GridBuilder::new_with_header(&schema);
        Grid {
            b,
            schema,
            _phantom: PhantomData,
        }
    }
}
impl<R: ?Sized, S: CellsSchema<Source = R>> Grid<R, S> {
    /// Append a row to the bottom of the grid.
    pub fn push(&mut self, source: &R) {
        self.b.push(|b| b.extend_with_schema(source, &self.schema));
    }

    /// Append a row separator to the bottom of the grid.
    pub fn push_separator(&mut self) {
        self.b.push_separator();
    }
}
impl<R, S: CellsSchema<Source = R>> Extend<R> for Grid<R, S> {
    fn extend<T: IntoIterator<Item = R>>(&mut self, iter: T) {
        for i in iter {
            self.push(&i);
        }
    }
}
impl<'a, R, S: CellsSchema<Source = R>> Extend<&'a R> for Grid<R, S> {
    fn extend<T: IntoIterator<Item = &'a R>>(&mut self, iter: T) {
        for i in iter {
            self.push(i);
        }
    }
}
impl<R: Cells> FromIterator<R> for Grid<R> {
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        let mut g = Self::new();
        g.extend(iter);
        g
    }
}
impl<'a, R: Cells> FromIterator<&'a R> for Grid<R> {
    fn from_iter<T: IntoIterator<Item = &'a R>>(iter: T) -> Self {
        let mut g = Self::new();
        g.extend(iter);
        g
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
