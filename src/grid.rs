use crate::cell::*;
use crate::grid_buf::*;
use std::cmp::max;
use std::fmt::*;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

/// A data structure that can be formatted into a row.
pub trait RowSource {
    /// Define column informations. see [`RowWriter`] for details.
    fn fmt_row(w: &mut RowWriter<&Self>);
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
///     fn fmt_row(&self, w: &mut RowWriter<&[u32]>) {
///         for i in 0..self.len {
///             w.column(i, |s| s[i]);
///         }
///     }
/// }
///
/// let mut g = Grid::new_with_schema(MyGridSchema { len: 3 });
/// g.push_row(&[1, 2, 3]);
/// g.push_row(&[4, 5, 6]);
///
/// print!("{}", g);
/// ```
/// Output:
/// ```text
///  0 | 1 | 2 |
/// ---|---|---|
///  1 | 2 | 3 |
///  4 | 5 | 6 |
/// ```
pub trait GridSchema<R: ?Sized> {
    /// Define column information. see [`RowWriter`] for details.
    fn fmt_row(&self, w: &mut RowWriter<&R>);
}

/// [`GridSchema`] implementation that use [`RowSource`].
pub struct RowSourceGridSchema;
impl<R: RowSource + ?Sized> GridSchema<R> for RowSourceGridSchema {
    fn fmt_row(&self, w: &mut RowWriter<&R>) {
        R::fmt_row(w);
    }
}

/// A builder used to create plain-text table from values.
///
/// # Examples
///
/// ```
/// use text_grid::*;
/// struct RowData {
///     a: u32,
///     b: u32,
/// }
/// impl RowSource for RowData {
///     fn fmt_row(w: &mut RowWriter<&Self>) {
///         w.column("a", |s| s.a);
///         w.column("b", |s| s.b);
///     }
/// }
///
/// let mut g = Grid::new();
/// g.push_row(&RowData { a: 300, b: 1 });
/// g.push_row(&RowData { a: 2, b: 200 });
///
/// print!("{}", g);
/// ```
///
/// Output:
/// ```text
///   a  |  b  |
/// -----|-----|
///  300 |   1 |
///    2 | 200 |
/// ```
pub struct Grid<R: ?Sized, S> {
    buf: GridBuf,
    schema: S,
    _phantom: PhantomData<fn(&R)>,
}

impl<R: RowSource + ?Sized> Default for Grid<R, RowSourceGridSchema> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: RowSource + ?Sized> Grid<R, RowSourceGridSchema> {
    /// Create a new `Grid` with [`RowSourceGridSchema`] and prepare header rows.
    pub fn new() -> Self {
        Self::new_with_schema(RowSourceGridSchema)
    }
}

impl<R: ?Sized, S: GridSchema<R>> Grid<R, S> {
    /// Create a new `Grid` with specified schema and prepare header rows.
    pub fn new_with_schema(schema: S) -> Self {
        let mut layout = LayoutWriter::new();
        schema.fmt_row(&mut RowWriter(RowWriterData::Layout(&mut layout)));
        layout.separators.pop();

        let mut buf = GridBuf::new();
        buf.set_column_separators(layout.separators);

        for target in 0..layout.depth_max {
            schema.fmt_row(&mut RowWriter(RowWriterData::Header(
                &mut HeaderWriter::new(buf.push_row(), target),
            )));
            buf.push_separator();
        }
        Grid {
            buf,
            schema,
            _phantom: PhantomData::default(),
        }
    }
}
impl<R: ?Sized, S: GridSchema<R>> Grid<R, S> {
    /// Append a row to the bottom of the grid.
    pub fn push_row(&mut self, source: &R) {
        self.schema
            .fmt_row(&mut RowWriter(RowWriterData::Body(BodyWriter {
                buf: &mut self.buf.push_row(),
                data: Some(source),
            })));
    }

    /// Append a row separator to the bottom of the grid.
    pub fn push_separator(&mut self) {
        self.buf.push_separator();
    }
}
impl<R: ?Sized, S> Display for Grid<R, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&self.buf, f)
    }
}
impl<R: ?Sized, S> Debug for Grid<R, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(&self.buf, f)
    }
}

/// Used to define column information.
///
/// - Use [`column`](Self::column) to create column.
/// - Use [`group`](Self::group) to create multi level header.
/// - Use [`content`](Self::content) to create shared header columns.
pub struct RowWriter<'a, 'b, T>(RowWriterData<'a, 'b, T>);

impl<'a, 'b, T> RowWriter<'a, 'b, T> {
    /// Define column group. Used to create multi row header.
    ///
    /// - header : Column group header's cell. If horizontal alignment is not specified, it is set to the center.
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
    /// impl RowSource for RowData {
    ///     fn fmt_row(w: &mut RowWriter<&Self>) {
    ///         w.column("a", |s| s.a);
    ///         w.group("b").with(|w| {
    ///             w.column("1", |s| s.b_1);
    ///             w.column("2", |s| s.b_2);
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
    ///
    /// ```
    ///
    /// Output:
    /// ```text
    ///   a  |    b     |
    /// -----|----------|
    ///      | 1  |  2  |
    /// -----|----|-----|
    ///  300 | 10 |  20 |
    ///  300 |  1 | 500 |
    ///  ```    
    pub fn group<'a0, C: CellSource>(&'a0 mut self, header: C) -> GroupGuard<'a0, 'a, 'b, T, C> {
        self.group_start();
        GroupGuard {
            w: self,
            header: Some(header),
        }
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
    /// impl RowSource for RowData {
    ///     fn fmt_row(w: &mut RowWriter<&Self>) {
    ///         w.column("a", |s| s.a);
    ///         w.group("b").with(|w| {
    ///             w.content(|s| s.b_1);
    ///             w.content(|_| " ");
    ///             w.content(|s| s.b_2);
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
    ///
    /// print!("{}", g);
    ///
    /// ```
    ///
    /// Output:
    /// ```text
    ///   a  |   b    |
    /// -----|--------|
    ///  300 | 10  20 |
    ///  300 |  1 500 |
    /// ```
    pub fn content<U: CellSource>(&mut self, f: impl FnOnce(&T) -> U) {
        match &mut self.0 {
            RowWriterData::Layout(w) => w.content(),
            RowWriterData::Header(w) => w.content(),
            RowWriterData::Body(w) => w.content(f),
        }
    }
    pub fn content_with_baseline<U: Display>(&mut self, baseline: &str, f: impl FnOnce(&T) -> U) {
        match &mut self.0 {
            RowWriterData::Layout(w) => w.content_with_baseline(),
            RowWriterData::Header(w) => w.content_with_baseline(),
            RowWriterData::Body(w) => w.content_with_baseline(baseline, f),
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
    /// impl RowSource for RowData {
    ///     fn fmt_row(w: &mut RowWriter<&Self>) {
    ///         w.column("a", |s| s.a);
    ///         w.column("b", |s| s.b);
    ///     }
    /// }
    ///
    /// let mut g = Grid::new();
    /// g.push_row(&RowData { a: 300, b: 1 });
    /// g.push_row(&RowData { a: 2, b: 200 });
    ///
    /// print!("{}", g);
    /// ```
    ///
    /// Output:
    /// ```text
    ///   a  |  b  |
    /// -----|-----|
    ///  300 |   1 |
    ///    2 | 200 |
    /// ```
    pub fn column<U: CellSource>(&mut self, header: impl CellSource, f: impl FnOnce(&T) -> U) {
        self.group(header).content(f);
    }

    pub fn column_with_baseline<U: Display>(
        &mut self,
        header: impl CellSource,
        baseline: &str,
        f: impl FnOnce(&T) -> U,
    ) {
        self.group(header).content_with_baseline(baseline, f);
    }

    /// Takes a closure and creates [`RowWriter`] whose source value was converted.
    pub fn map<'x, U: 'x>(&'x mut self, f: impl FnOnce(&T) -> U) -> RowWriter<'x, 'b, U> {
        RowWriter(match &mut self.0 {
            RowWriterData::Layout(w) => RowWriterData::Layout(w),
            RowWriterData::Header(w) => RowWriterData::Header(w),
            RowWriterData::Body(w) => RowWriterData::Body(BodyWriter {
                buf: w.buf,
                data: w.data.as_ref().map(f),
            }),
        })
    }

    pub fn as_ref<'x>(&'x mut self) -> RowWriter<'x, 'b, &'x T> {
        RowWriter(match &mut self.0 {
            RowWriterData::Layout(w) => RowWriterData::Layout(w),
            RowWriterData::Header(w) => RowWriterData::Header(w),
            RowWriterData::Body(w) => RowWriterData::Body(BodyWriter {
                buf: w.buf,
                data: w.data.as_ref(),
            }),
        })
    }

    /// Creates [`RowWriter`] which uses a closure to determine if an content should be outputed.
    pub fn filter(&mut self, f: impl FnOnce(&T) -> bool) -> RowWriter<'_, 'b, &T> {
        RowWriter(match &mut self.0 {
            RowWriterData::Layout(w) => RowWriterData::Layout(w),
            RowWriterData::Header(w) => RowWriterData::Header(w),
            RowWriterData::Body(w) => RowWriterData::Body(BodyWriter {
                buf: w.buf,
                data: w.data.as_ref().filter(|data| f(data)),
            }),
        })
    }

    /// Creates [`RowWriter`] that both filters and maps.
    pub fn filter_map<'a0, U: 'a0>(
        &'a0 mut self,
        f: impl FnOnce(&T) -> Option<U>,
    ) -> RowWriter<'a0, 'b, U> {
        RowWriter(match &mut self.0 {
            RowWriterData::Layout(w) => RowWriterData::Layout(w),
            RowWriterData::Header(w) => RowWriterData::Header(w),
            RowWriterData::Body(w) => RowWriterData::Body(BodyWriter {
                buf: w.buf,
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
            RowWriterData::Layout(w) => w.group_start(),
            RowWriterData::Header(w) => w.group_start(),
            RowWriterData::Body(w) => w.group_start(),
        }
    }
    fn group_end(&mut self, header: impl CellSource) {
        match &mut self.0 {
            RowWriterData::Layout(w) => w.group_end(),
            RowWriterData::Header(w) => w.group_end(header),
            RowWriterData::Body(w) => w.group_end(),
        }
    }
}

enum RowWriterData<'a, 'b, T> {
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
    fn content_with_baseline(&mut self) {
        self.separators.push(false);
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
    buf: RowBuf<'b>,
    depth: usize,
    target: usize,
    column: usize,
    column_last: usize,
}
impl<'b> HeaderWriter<'b> {
    fn new(buf: RowBuf<'b>, target: usize) -> Self {
        Self {
            buf,
            depth: 0,
            target,
            column: 0,
            column_last: 0,
        }
    }

    fn push_cell(&mut self, cell: impl CellSource) {
        let colspan = self.column - self.column_last;
        self.buf.push_with_colspan(cell, colspan);
        self.column_last = self.column;
    }
    fn content(&mut self) {
        self.column += 1;
    }
    fn content_with_baseline(&mut self) {
        self.column += 2;
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
    buf: &'a mut RowBuf<'b>,
    data: Option<T>,
}
impl<T> BodyWriter<'_, '_, T> {
    fn content<U: CellSource>(&mut self, f: impl FnOnce(&T) -> U) {
        if let Some(data) = &self.data {
            self.buf.push(f(data));
        } else {
            self.buf.push("");
        }
    }

    fn content_with_baseline<U: Display>(&mut self, baseline: &str, f: impl FnOnce(&T) -> U) {
        if let Some(data) = &self.data {
            let s = f(data).to_string();
            let b = s.find(baseline).unwrap_or(s.len());
            let (left, right) = s.split_at(b);
            self.buf.push(cell(left).right());
            self.buf.push(cell(right).left());
        } else {
            self.buf.push("");
            self.buf.push("");
        }
    }
    fn group_start(&mut self) {}
    fn group_end(&mut self) {}
}

pub struct GroupGuard<'a, 'b, 'c, T, C: CellSource> {
    w: &'a mut RowWriter<'b, 'c, T>,
    header: Option<C>,
}

impl<'a, 'b, 'c, T, C: CellSource> Deref for GroupGuard<'a, 'b, 'c, T, C> {
    type Target = RowWriter<'b, 'c, T>;
    fn deref(&self) -> &Self::Target {
        self.w
    }
}

impl<'a, 'b, 'c, T, C: CellSource> DerefMut for GroupGuard<'a, 'b, 'c, T, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.w
    }
}

impl<T, C: CellSource> Drop for GroupGuard<'_, '_, '_, T, C> {
    fn drop(&mut self) {
        self.w.group_end(self.header.take());
    }
}
