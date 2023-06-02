use crate::cell::*;
use crate::grid_builder::*;
use std::cmp::max;
use std::fmt::*;
use std::marker::PhantomData;

/// A data structure that can be formatted into cells.
pub trait ColumnSource {
    /// Define columns. see [`ColumnFormatter`] for details.
    fn fmt(f: &mut ColumnFormatter<&Self>);
}

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
///     fn fmt(&self, f: &mut ColumnFormatter<&[u32]>) {
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
    /// Define column information. see [`ColumnFormatter`] for details.
    fn fmt(&self, f: &mut ColumnFormatter<&R>);
}

/// [`GridSchema`] implementation that use [`ColumnSource`].
pub struct GridSchemaByColumnSource;
impl<R: ColumnSource + ?Sized> GridSchema<R> for GridSchemaByColumnSource {
    fn fmt(&self, f: &mut ColumnFormatter<&R>) {
        R::fmt(f);
    }
}

/// A builder used to create plain-text table from row values.
///
/// Generate a table using the columns defined by [`ColumnFormatter`].
///
/// # Examples
///
/// ```
/// use text_grid::*;
/// struct RowData {
///     a: u32,
///     b: u32,
/// }
/// impl ColumnSource for RowData {
///     fn fmt(f: &mut ColumnFormatter<&Self>) {
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

impl<R: ColumnSource + ?Sized> Default for Grid<R, GridSchemaByColumnSource> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: ColumnSource + ?Sized> Grid<R, GridSchemaByColumnSource> {
    /// Create a new `Grid` with [`GridSchemaByColumnSource`] and prepare header rows.
    pub fn new() -> Self {
        Self::new_with_schema(GridSchemaByColumnSource)
    }
}

impl<R: ?Sized, S: GridSchema<R>> Grid<R, S> {
    /// Create a new `Grid` with specified schema and prepare header rows.
    pub fn new_with_schema(schema: S) -> Self {
        let mut layout = LayoutWriter::new();
        schema.fmt(&mut ColumnFormatter(ColumnFormatterData::Layout(
            &mut layout,
        )));
        layout.separators.pop();

        let mut b = GridBuilder::new();
        b.set_column_separators(layout.separators);

        for target in 0..layout.depth_max {
            schema.fmt(&mut ColumnFormatter(ColumnFormatterData::Header(
                &mut HeaderWriter::new(b.push_row(), target),
            )));
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
        self.schema
            .fmt(&mut ColumnFormatter(ColumnFormatterData::Body(
                BodyWriter {
                    b: &mut self.b.push_row(),
                    data: Some(source),
                },
            )));
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
pub struct ColumnFormatter<'a, 'b, T>(ColumnFormatterData<'a, 'b, T>);

impl<'a, 'b, T> ColumnFormatter<'a, 'b, T> {
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
    /// impl ColumnSource for RowData {
    ///     fn fmt(f: &mut ColumnFormatter<&Self>) {
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
    pub fn group(
        &mut self,
        header: impl CellSource,
        f: impl FnOnce(&mut ColumnFormatter<'_, 'b, T>),
    ) {
        self.group_start();
        f(self);
        self.group_end(header);
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
    /// impl ColumnSource for RowData {
    ///     fn fmt(f: &mut ColumnFormatter<&Self>) {
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
    pub fn content<U: ColumnSource>(&mut self, f: impl FnOnce(&T) -> U) {
        U::fmt(&mut self.map(f).as_ref())
    }

    fn content_raw<U: CellSource>(&mut self, f: impl FnOnce(&T) -> U) {
        match &mut self.0 {
            ColumnFormatterData::Layout(w) => w.content(),
            ColumnFormatterData::Header(w) => w.content(),
            ColumnFormatterData::Body(w) => w.content(f),
        }
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
    /// impl ColumnSource for RowData {
    ///     fn fmt(f: &mut ColumnFormatter<&Self>) {
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
    pub fn column<U: ColumnSource>(&mut self, header: impl CellSource, f: impl FnOnce(&T) -> U) {
        self.group(header, |cf| cf.content(f));
    }

    /// Creates a [`ColumnFormatter`] whose source value was converted.
    pub fn map<'x, U: 'x>(&'x mut self, f: impl FnOnce(&T) -> U) -> ColumnFormatter<'x, 'b, U> {
        ColumnFormatter(match &mut self.0 {
            ColumnFormatterData::Layout(w) => ColumnFormatterData::Layout(w),
            ColumnFormatterData::Header(w) => ColumnFormatterData::Header(w),
            ColumnFormatterData::Body(w) => ColumnFormatterData::Body(BodyWriter {
                b: w.b,
                data: w.data.as_ref().map(f),
            }),
        })
    }

    /// Creates a [`ColumnFormatter`] whose source value was converted to reference.
    pub fn as_ref<'x>(&'x mut self) -> ColumnFormatter<'x, 'b, &'x T> {
        ColumnFormatter(match &mut self.0 {
            ColumnFormatterData::Layout(w) => ColumnFormatterData::Layout(w),
            ColumnFormatterData::Header(w) => ColumnFormatterData::Header(w),
            ColumnFormatterData::Body(w) => ColumnFormatterData::Body(BodyWriter {
                b: w.b,
                data: w.data.as_ref(),
            }),
        })
    }

    /// Creates a [`ColumnFormatter`] that outputs the body cell only when the source value satisfies the condition.
    pub fn filter(&mut self, f: impl FnOnce(&T) -> bool) -> ColumnFormatter<'_, 'b, &T> {
        ColumnFormatter(match &mut self.0 {
            ColumnFormatterData::Layout(w) => ColumnFormatterData::Layout(w),
            ColumnFormatterData::Header(w) => ColumnFormatterData::Header(w),
            ColumnFormatterData::Body(w) => ColumnFormatterData::Body(BodyWriter {
                b: w.b,
                data: w.data.as_ref().filter(|data| f(data)),
            }),
        })
    }

    /// Creates a [`ColumnFormatter`] that both filters and maps.
    pub fn filter_map<'a0, U: 'a0>(
        &'a0 mut self,
        f: impl FnOnce(&T) -> Option<U>,
    ) -> ColumnFormatter<'a0, 'b, U> {
        ColumnFormatter(match &mut self.0 {
            ColumnFormatterData::Layout(w) => ColumnFormatterData::Layout(w),
            ColumnFormatterData::Header(w) => ColumnFormatterData::Header(w),
            ColumnFormatterData::Body(w) => ColumnFormatterData::Body(BodyWriter {
                b: w.b,
                data: w.data.as_ref().and_then(f),
            }),
        })
    }

    /// Apply `f` to self.
    pub fn with(&mut self, f: impl Fn(&mut Self)) {
        f(self);
    }

    fn group_start(&mut self) {
        match &mut self.0 {
            ColumnFormatterData::Layout(w) => w.group_start(),
            ColumnFormatterData::Header(w) => w.group_start(),
            ColumnFormatterData::Body(w) => w.group_start(),
        }
    }
    fn group_end(&mut self, header: impl CellSource) {
        match &mut self.0 {
            ColumnFormatterData::Layout(w) => w.group_end(),
            ColumnFormatterData::Header(w) => w.group_end(header),
            ColumnFormatterData::Body(w) => w.group_end(),
        }
    }
}

enum ColumnFormatterData<'a, 'b, T> {
    Layout(&'a mut LayoutWriter),
    Header(&'a mut HeaderWriter<'b>),
    Body(BodyWriter<'a, 'b, T>),
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
    fn content(&mut self) {
        self.separators.push(false);
    }
    fn group_start(&mut self) {
        self.set_separator();
        self.depth += 1;
        self.depth_max = max(self.depth_max, self.depth);
    }
    fn group_end(&mut self) {
        self.depth -= 1;
        self.set_separator()
    }
    fn set_separator(&mut self) {
        if let Some(last) = self.separators.last_mut() {
            *last = true;
        }
    }
}

struct HeaderWriter<'b> {
    b: RowBuilder<'b>,
    depth: usize,
    target: usize,
    column: usize,
    column_last: usize,
}
impl<'b> HeaderWriter<'b> {
    fn new(b: RowBuilder<'b>, target: usize) -> Self {
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
    fn content(&mut self) {
        self.column += 1;
    }
    fn group_start(&mut self) {
        if self.depth <= self.target {
            self.push_cell(Cell::empty());
        }
        self.depth += 1;
    }
    fn group_end(&mut self, header: impl CellSource) {
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
impl Drop for HeaderWriter<'_> {
    fn drop(&mut self) {
        self.push_cell("");
    }
}

struct BodyWriter<'a, 'b, T> {
    b: &'a mut RowBuilder<'b>,
    data: Option<T>,
}
impl<T> BodyWriter<'_, '_, T> {
    fn content<U: CellSource>(&mut self, f: impl FnOnce(&T) -> U) {
        if let Some(data) = &self.data {
            self.b.push(f(data));
        } else {
            self.b.push("");
        }
    }

    fn group_start(&mut self) {}
    fn group_end(&mut self) {}
}

impl<T: CellSource> ColumnSource for T {
    fn fmt(f: &mut ColumnFormatter<&Self>) {
        f.content_raw(|&x| x);
    }
}
