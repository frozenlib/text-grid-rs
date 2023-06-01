use crate::cell::*;
use crate::grid_buf::*;
use crate::row_write::*;
use std::cmp::max;
use std::fmt::*;
use std::marker::PhantomData;

/// A data structure that can be formatted into a row.
pub trait RowSource {
    /// Define column informations. see [`RowWrite`] for details.
    fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>)
    where
        Self: 'a;
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
///     fn fmt_row<'a>(&self, w: &mut impl RowWrite<Source = &'a [u32]>) {
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
    /// Define column information. see [`RowWrite`] for details.
    fn fmt_row<'a>(&self, w: &mut impl RowWrite<Source = &'a R>)
    where
        R: 'a;
}

/// [`GridSchema`] implementation that use [`RowSource`].
pub struct RowSourceGridSchema;
impl<R: RowSource + ?Sized> GridSchema<R> for RowSourceGridSchema {
    fn fmt_row<'a>(&self, w: &mut impl RowWrite<Source = &'a R>)
    where
        R: 'a,
    {
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
///     fn fmt_row<'a>(w: &mut impl RowWrite<Source=&'a Self>) {
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
        schema.fmt_row(&mut layout);
        layout.separators.pop();

        let mut buf = GridBuf::new();
        buf.set_column_separators(layout.separators);

        for target in 0..layout.depth_max {
            schema.fmt_row(&mut HeaderWriter::new(buf.push_row(), target));
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
        let mut writer = RowWriter {
            source,
            row: self.buf.push_row(),
        };
        self.schema.fmt_row(&mut writer);
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

struct LayoutWriter<S> {
    depth: usize,
    depth_max: usize,
    separators: Vec<bool>,
    _phantom: PhantomData<fn(S)>,
}
impl<S> LayoutWriter<S> {
    fn new() -> Self {
        LayoutWriter {
            depth: 0,
            depth_max: 0,
            separators: Vec::new(),
            _phantom: PhantomData::default(),
        }
    }
    fn set_separator(&mut self) {
        if let Some(last) = self.separators.last_mut() {
            *last = true;
        }
    }
}
impl<S> RowWrite for LayoutWriter<S> {
    type Source = S;
    fn content<T: CellSource>(&mut self, _f: impl FnOnce(S) -> T) {
        assert!(self.depth != 0);
        self.separators.push(false);
    }
    fn content_with_baseline<T: Display>(
        &mut self,
        _baseline: &str,
        _f: impl FnOnce(Self::Source) -> T,
    ) {
        assert!(self.depth != 0);
        self.separators.push(false);
        self.separators.push(false);
    }
}
impl<S> RowWriteCore for LayoutWriter<S> {
    fn group_start(&mut self) {
        self.set_separator();
        self.depth += 1;
        self.depth_max = max(self.depth_max, self.depth);
    }
    fn group_end(&mut self, _header: impl CellSource) {
        self.depth -= 1;
        self.set_separator()
    }
}

struct HeaderWriter<'a, S: ?Sized> {
    row: RowBuf<'a>,
    depth: usize,
    target: usize,
    column: usize,
    column_last: usize,
    _phantom: PhantomData<fn(S)>,
}
impl<'a, S: ?Sized> HeaderWriter<'a, S> {
    fn new(row: RowBuf<'a>, target: usize) -> Self {
        HeaderWriter {
            row,
            depth: 0,
            target,
            column: 0,
            column_last: 0,
            _phantom: PhantomData::default(),
        }
    }
    fn push_cell(&mut self, cell: impl CellSource) {
        let colspan = self.column - self.column_last;
        self.row.push_with_colspan(cell, colspan);
        self.column_last = self.column;
    }
}
impl<'a, S: ?Sized> Drop for HeaderWriter<'a, S> {
    fn drop(&mut self) {
        self.push_cell("");
    }
}

impl<'a, S: 'a + ?Sized> RowWrite for HeaderWriter<'a, S> {
    type Source = &'a S;
    fn content<T: CellSource>(&mut self, _f: impl FnOnce(Self::Source) -> T) {
        assert!(self.depth != 0);
        self.column += 1;
    }
    fn content_with_baseline<T: Display>(
        &mut self,
        _baseline: &str,
        _f: impl FnOnce(Self::Source) -> T,
    ) {
        assert!(self.depth != 0);
        self.column += 2;
    }
}
impl<'a, S: 'a + ?Sized> RowWriteCore for HeaderWriter<'a, S> {
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

struct RowWriter<'a, R: ?Sized> {
    source: &'a R,
    row: RowBuf<'a>,
}
impl<'a, R: ?Sized> RowWrite for RowWriter<'a, R> {
    type Source = &'a R;
    fn content<T: CellSource>(&mut self, f: impl FnOnce(Self::Source) -> T) {
        self.row.push(f(self.source));
    }

    fn content_with_baseline<T: Display>(
        &mut self,
        baseline: &str,
        f: impl FnOnce(Self::Source) -> T,
    ) {
        let s = f(self.source).to_string();
        let b = s.find(baseline).unwrap_or(s.len());
        let (left, right) = s.split_at(b);
        self.row.push(cell(left).right());
        self.row.push(cell(right).left());
    }
}
impl<'a, R: ?Sized> RowWriteCore for RowWriter<'a, R> {
    fn group_start(&mut self) {}
    fn group_end(&mut self, _header: impl CellSource) {}
}
