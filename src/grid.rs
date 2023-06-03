use crate::cell::*;
use crate::grid_builder::*;
use std::cmp::max;
use std::fmt::*;
use std::marker::PhantomData;

/// A data structure that can be formatted into cells.
pub trait CellsSource {
    /// Define columns. see [`CellsFormatter`] for details.
    fn fmt(f: &mut CellsFormatter<&Self>);
}
impl<T: ?Sized + CellsSource> CellsSource for &T {
    fn fmt(f: &mut CellsFormatter<&Self>) {
        T::fmt(&mut f.map(|x| **x));
    }
}
impl<T: ?Sized + CellsSource> CellsSource for &mut T {
    fn fmt(f: &mut CellsFormatter<&Self>) {
        T::fmt(&mut f.map(|x| **x as &T));
    }
}

impl<T: CellsSource, const N: usize> CellsSource for [T; N] {
    fn fmt(f: &mut CellsFormatter<&Self>) {
        for i in 0..N {
            T::fmt(&mut f.map(|x| &x[i]));
        }
    }
}

macro_rules! impl_grid_source_for_tuple {
    ($($idx:tt : $ty:ident,)*) => {
        impl<$($ty),*> CellsSource for ($($ty,)*) where $($ty: CellsSource),* {
            fn fmt(f: &mut CellsFormatter<&Self>) {
                $(
                    $ty::fmt(&mut f.map(|x| &x.$idx));
                )*
            }
        }
    };
}
impl_grid_source_for_tuple!(0: T0,);
impl_grid_source_for_tuple!(0: T0, 1: T1,);
impl_grid_source_for_tuple!(0: T0, 1: T1, 2: T2,);
impl_grid_source_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3,);
impl_grid_source_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4,);
impl_grid_source_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5,);
impl_grid_source_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6,);
impl_grid_source_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7,);
impl_grid_source_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
);
impl_grid_source_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
);
impl_grid_source_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
);
impl_grid_source_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
    11: T11,
);
impl_grid_source_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
    11: T11,
    12: T12,
);
impl_grid_source_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
    11: T11,
    12: T12,
    13: T13,
);
impl_grid_source_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
    11: T11,
    12: T12,
    13: T13,
    14: T14,
);
impl_grid_source_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
    11: T11,
    12: T12,
    13: T13,
    14: T14,
    15: T15,
);

/// Columns definition.
///
/// # Examples
/// ```
/// use text_grid::*;
///
/// struct MyGridSchema {
///     len: usize,
/// }
///
/// impl GridSchema<[u32]> for MyGridSchema {
///     fn fmt(&self, f: &mut CellsFormatter<&[u32]>) {
///         for i in 0..self.len {
///             f.column(i, |s| s[i]);
///         }
///     }
/// }
///
/// let mut g = Grid::new_with_schema(MyGridSchema { len: 3 });
/// g.push_row(&[1, 2, 3]);
/// g.push_row(&[4, 5, 6]);
///
/// assert_eq!(format!("\n{g}"), r#"
///  0 | 1 | 2 |
/// ---|---|---|
///  1 | 2 | 3 |
///  4 | 5 | 6 |
/// "#);
/// ```
pub trait GridSchema<R: ?Sized> {
    /// Define column information. see [`CellsFormatter`] for details.
    fn fmt(&self, f: &mut CellsFormatter<&R>);
}

/// [`GridSchema`] implementation that use [`CellsSource`].
pub struct DefaultGridSchema;
impl<R: CellsSource + ?Sized> GridSchema<R> for DefaultGridSchema {
    fn fmt(&self, f: &mut CellsFormatter<&R>) {
        R::fmt(f);
    }
}

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
/// g.push_row(&RowData { a: 300, b: 1 });
/// g.push_row(&RowData { a: 2, b: 200 });
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

impl<R: CellsSource + ?Sized> Default for Grid<R, DefaultGridSchema> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: CellsSource + ?Sized> Grid<R, DefaultGridSchema> {
    /// Create a new `Grid` with [`DefaultGridSchema`] and prepare header rows.
    pub fn new() -> Self {
        Self::new_with_schema(DefaultGridSchema)
    }
}

impl<R: ?Sized, S: GridSchema<R>> Grid<R, S> {
    /// Create a new `Grid` with specified schema and prepare header rows.
    pub fn new_with_schema(schema: S) -> Self {
        let mut layout = LayoutWriter::new();
        schema.fmt(&mut CellsFormatter {
            w: &mut layout,
            d: None,
        });
        layout.separators.pop();

        let mut b = GridBuilder::new();
        b.set_column_separators(layout.separators);

        for target in 0..layout.depth_max {
            b.push_row(|b| {
                schema.fmt(&mut CellsFormatter {
                    w: &mut HeaderWriter::new(b, target),
                    d: None,
                })
            });
            b.push_separator();
        }
        Grid {
            b,
            schema,
            _phantom: PhantomData::default(),
        }
    }
}
impl<R: ?Sized, S: GridSchema<R>> Grid<R, S> {
    /// Append a row to the bottom of the grid.
    pub fn push_row(&mut self, source: &R) {
        self.b.push_row(|b| {
            self.schema.fmt(&mut CellsFormatter {
                w: &mut BodyWriter(b),
                d: Some(source),
            })
        });
    }

    /// Append a row separator to the bottom of the grid.
    pub fn push_separator(&mut self) {
        self.b.push_separator();
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

/// Used to define columns.
///
/// - Use [`column`](Self::column) to create column.
/// - Use [`group`](Self::group) to create multi level header.
/// - Use [`content`](Self::content) to create shared header columns.
pub struct CellsFormatter<'a, T> {
    w: &'a mut dyn RowWrite,
    d: Option<T>,
}

impl<'a, T> CellsFormatter<'a, T> {
    /// Define column group. Used to create multi row header.
    ///
    /// - header : Column group header's cell. If horizontal alignment is not specified, it is set to the center.
    /// - f : A function to define columns in the group.
    ///
    /// # Examples
    ///
    /// ```
    /// use text_grid::*;
    /// struct RowData {
    ///     a: u32,
    ///     b_1: u32,
    ///     b_2: u32,
    /// }
    /// impl CellsSource for RowData {
    ///     fn fmt(f: &mut CellsFormatter<&Self>) {
    ///         f.column("a", |s| s.a);
    ///         f.group("b", |f| {
    ///             f.column("1", |s| s.b_1);
    ///             f.column("2", |s| s.b_2);
    ///         });
    ///     }
    /// }
    ///
    /// let mut g = Grid::new();
    /// g.push_row(&RowData {
    ///     a: 300,
    ///     b_1: 10,
    ///     b_2: 20,
    /// });
    /// g.push_row(&RowData {
    ///     a: 300,
    ///     b_1: 1,
    ///     b_2: 500,
    /// });
    /// assert_eq!(format!("\n{g}"), r#"
    ///   a  |    b     |
    /// -----|----------|
    ///      | 1  |  2  |
    /// -----|----|-----|
    ///  300 | 10 |  20 |
    ///  300 |  1 | 500 |
    /// "#);
    /// ```
    pub fn group(&mut self, header: impl CellSource, f: impl FnOnce(&mut CellsFormatter<T>)) {
        self.w.group_start();
        f(self);
        self.w.group_end(&header);
    }

    /// Define column content. Used to create shared header column.
    ///
    /// - f : A function to obtain cell.
    ///
    /// # Examples
    ///
    /// ```
    /// use text_grid::*;
    /// struct RowData {
    ///     a: u32,
    ///     b_1: u32,
    ///     b_2: u32,
    /// }
    /// impl CellsSource for RowData {
    ///     fn fmt(f: &mut CellsFormatter<&Self>) {
    ///         f.column("a", |s| s.a);
    ///         f.group("b", |f| {
    ///             f.content(|s| s.b_1);
    ///             f.content(|_| " ");
    ///             f.content(|s| s.b_2);
    ///         });
    ///     }
    /// }
    ///
    /// let mut g = Grid::new();
    /// g.push_row(&RowData {
    ///     a: 300,
    ///     b_1: 10,
    ///     b_2: 20,
    /// });
    /// g.push_row(&RowData {
    ///     a: 300,
    ///     b_1: 1,
    ///     b_2: 500,
    /// });
    /// assert_eq!(format!("\n{g}"), r#"
    ///   a  |   b    |
    /// -----|--------|
    ///  300 | 10  20 |
    ///  300 |  1 500 |
    /// "#);
    /// ```
    pub fn content<U: CellsSource>(&mut self, f: impl FnOnce(&T) -> U) {
        U::fmt(&mut self.map(f).as_ref())
    }

    pub fn content_cell<U: CellSource>(&mut self, f: impl FnOnce(&T) -> U) {
        self.w.content(
            self.d
                .as_ref()
                .map(f)
                .as_ref()
                .map(|x| x as &dyn CellSource),
        );
    }

    /// Define column.
    ///
    /// - header : Column header's cell. If horizontal alignment is not specified, it is set to the center.
    /// - f : A function to obtain cell.
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
    /// g.push_row(&RowData { a: 300, b: 1 });
    /// g.push_row(&RowData { a: 2, b: 200 });
    /// assert_eq!(format!("\n{g}"), r#"
    ///   a  |  b  |
    /// -----|-----|
    ///  300 |   1 |
    ///    2 | 200 |
    /// "#);
    /// ```
    pub fn column<U: CellsSource>(&mut self, header: impl CellSource, f: impl FnOnce(&T) -> U) {
        self.group(header, |cf| cf.content(f));
    }

    /// Creates a [`CellsFormatter`] whose source value was converted.
    pub fn map<'x, U: 'x>(&'x mut self, f: impl FnOnce(&T) -> U) -> CellsFormatter<'x, U> {
        CellsFormatter {
            w: self.w,
            d: self.d.as_ref().map(f),
        }
    }

    /// Creates a [`CellsFormatter`] whose source value was converted to reference.
    pub fn as_ref(&mut self) -> CellsFormatter<&T> {
        CellsFormatter {
            w: self.w,
            d: self.d.as_ref(),
        }
    }

    /// Creates a [`CellsFormatter`] that outputs the body cell only when the source value satisfies the condition.
    pub fn filter(&mut self, f: impl FnOnce(&T) -> bool) -> CellsFormatter<&T> {
        CellsFormatter {
            w: self.w,
            d: self.d.as_ref().filter(|data| f(data)),
        }
    }

    /// Creates a [`CellsFormatter`] that both filters and maps.
    pub fn filter_map<'x, U: 'x>(
        &'x mut self,
        f: impl FnOnce(&T) -> Option<U>,
    ) -> CellsFormatter<'x, U> {
        CellsFormatter {
            w: self.w,
            d: self.d.as_ref().and_then(f),
        }
    }

    /// Apply `f` to self.
    pub fn with(&mut self, f: impl Fn(&mut Self)) {
        f(self);
    }
}

trait RowWrite {
    fn content(&mut self, cell: Option<&dyn CellSource>);
    fn group_start(&mut self);
    fn group_end(&mut self, header: &dyn CellSource);
}

struct LayoutWriter {
    depth: usize,
    depth_max: usize,
    separators: Vec<bool>,
}
impl LayoutWriter {
    fn new() -> Self {
        Self {
            depth: 0,
            depth_max: 0,
            separators: Vec::new(),
        }
    }
    fn set_separator(&mut self) {
        if let Some(last) = self.separators.last_mut() {
            *last = true;
        }
    }
}
impl RowWrite for LayoutWriter {
    fn content(&mut self, _cell: Option<&dyn CellSource>) {
        self.separators.push(false);
    }

    fn group_start(&mut self) {
        self.set_separator();
        self.depth += 1;
        self.depth_max = max(self.depth_max, self.depth);
    }

    fn group_end(&mut self, _header: &dyn CellSource) {
        self.depth -= 1;
        self.set_separator()
    }
}

struct HeaderWriter<'a, 'b> {
    b: &'a mut RowBuilder<'b>,
    depth: usize,
    target: usize,
    column: usize,
    column_last: usize,
}
impl<'a, 'b> HeaderWriter<'a, 'b> {
    fn new(b: &'a mut RowBuilder<'b>, target: usize) -> Self {
        Self {
            b,
            depth: 0,
            target,
            column: 0,
            column_last: 0,
        }
    }

    fn push_cell(&mut self, cell: impl CellSource) {
        let colspan = self.column - self.column_last;
        self.b.push_with_colspan(cell, colspan);
        self.column_last = self.column;
    }
}
impl RowWrite for HeaderWriter<'_, '_> {
    fn content(&mut self, _cell: Option<&dyn CellSource>) {
        self.column += 1;
    }
    fn group_start(&mut self) {
        if self.depth <= self.target {
            self.push_cell(Cell::empty());
        }
        self.depth += 1;
    }
    fn group_end(&mut self, header: &dyn CellSource) {
        self.depth -= 1;
        if self.depth == self.target {
            let style = CellStyle {
                align_h: Some(HorizontalAlignment::Center),
            };
            let header = Cell::new(header).with_base_style(style);
            self.push_cell(header);
        }
    }
}
impl Drop for HeaderWriter<'_, '_> {
    fn drop(&mut self) {
        self.push_cell("");
    }
}

struct BodyWriter<'a, 'b>(&'a mut RowBuilder<'b>);

impl RowWrite for BodyWriter<'_, '_> {
    fn content(&mut self, cell: Option<&dyn CellSource>) {
        if let Some(cell) = cell {
            self.0.push(cell);
        } else {
            self.0.push("");
        }
    }
    fn group_start(&mut self) {}
    fn group_end(&mut self, _header: &dyn CellSource) {}
}
