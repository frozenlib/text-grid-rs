use crate::cell::*;
use crate::grid_buf::*;
use std::cmp::max;
use std::fmt::*;
use std::marker::PhantomData;

/// Used to define column information within [`RowSource::fmt_row`].
///
/// - Use [`Self::column`] to create column.
/// - Use [`Self::group`] to create multi level header.
/// - Use [`Self::content`] to create shared header columns.
pub trait RowWrite<'a, S: 'a + ?Sized> {
    /// Define column.
    ///
    /// - header : Column header's cell. If horizontal alignment is not specified, it is set to the center.
    /// - f : A function to obtain [`CellSource`] from [`RowSource`].
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
    ///     fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>) {
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
    fn column<T: CellSource>(&mut self, header: impl CellSource, f: impl FnOnce(&'a S) -> T) {
        self.group(header, |s| s.content(f));
    }

    /// Define column group. Used to create multi row header.
    ///
    /// - header : Column group header's cell. If horizontal alignment is not specified, it is set to the center.
    /// - f : A function to define group contents.
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
    ///     fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>) {
    ///         w.column("a", |s| s.a);
    ///         w.group("b", |w| {
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
    /// print!("{}", g);
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
    fn group(&mut self, header: impl CellSource, f: impl FnOnce(&mut Self));

    /// Define column content. Used to create shared header column.
    ///
    /// - f : A function to obtain [`CellSource`] from [`RowSource`].
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
    ///     fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>) {
    ///         w.column("a", |s| s.a);
    ///         w.group("b", |w| {
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
    /// ```
    ///
    /// Output:
    /// ```text
    ///   a  |   b    |
    /// -----|--------|
    ///  300 | 10  20 |
    ///  300 |  1 500 |
    /// ```
    fn content<T: CellSource>(&mut self, f: impl FnOnce(&'a S) -> T);
}

/// A data structure that can be formatted into row.
pub trait RowSource {
    /// Define column informations. see [`RowWrite`] for details.
    ///
    fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>)
    where
        Self: 'a;
}

/// A builder used to create plain-text table from struct that implement [`RowSource`].
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
///     fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>) {
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
pub struct Grid<S> {
    buf: GridBuf,
    _phantom: PhantomData<Fn(&S)>,
}

impl<T: RowSource> Grid<T> {
    /// Create a new `Grid` and prepare header rows.
    pub fn new() -> Self {
        let mut layout = LayoutWriter::new();
        T::fmt_row(&mut layout);
        layout.separators.pop();

        let mut buf = GridBuf::new();
        buf.set_column_separators(layout.separators);

        for target in 0..layout.depth_max {
            T::fmt_row(&mut HeaderWriter::new(buf.push_row(), target));
            buf.push_separator();
        }
        Grid {
            buf,
            _phantom: PhantomData::default(),
        }
    }

    /// Append a row to the bottom of the grid.
    pub fn push_row(&mut self, source: &T) {
        let mut writer = RowWriter {
            source,
            row: self.buf.push_row(),
        };
        T::fmt_row(&mut writer);
    }

    /// Append a row separator to the bottom of the grid.
    pub fn push_separator(&mut self) {
        self.buf.push_separator();
    }
}
impl<S> Display for Grid<S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.buf.fmt(f)
    }
}

struct LayoutWriter {
    depth: usize,
    depth_max: usize,
    separators: Vec<bool>,
}
impl LayoutWriter {
    fn new() -> Self {
        LayoutWriter {
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
impl<'a, S: 'a> RowWrite<'a, S> for LayoutWriter {
    fn group(&mut self, _header: impl CellSource, f: impl FnOnce(&mut Self)) {
        self.set_separator();
        self.depth += 1;
        self.depth_max = max(self.depth_max, self.depth);
        f(self);
        self.depth -= 1;
        self.set_separator()
    }
    fn content<T: CellSource>(&mut self, _f: impl FnOnce(&'a S) -> T) {
        assert!(self.depth != 0);
        self.separators.push(false);
    }
}

struct HeaderWriter<'a> {
    row: RowBuf<'a>,
    depth: usize,
    target: usize,
    column: usize,
    column_last: usize,
}
impl<'a> HeaderWriter<'a> {
    fn new(row: RowBuf<'a>, target: usize) -> Self {
        HeaderWriter {
            row,
            depth: 0,
            target,
            column: 0,
            column_last: 0,
        }
    }
    fn push_cell(&mut self, cell: impl CellSource) {
        let colspan = self.column - self.column_last;
        self.row.push_with_colspan(cell, colspan);
        self.column_last = self.column;
    }
}
impl<'a, S: 'a> RowWrite<'a, S> for HeaderWriter<'a> {
    fn group(&mut self, header: impl CellSource, f: impl FnOnce(&mut Self)) {
        if self.depth <= self.target {
            self.push_cell(Cell::empty());
        }
        self.depth += 1;
        f(self);
        self.depth -= 1;

        if self.depth == self.target {
            let mut style = CellStyle::default();
            style.align_h = Some(HorizontalAlignment::Center);

            let header = Cell::new(header).with_base_style(style);
            self.push_cell(header);
        }
    }
    fn content<T: CellSource>(&mut self, _f: impl FnOnce(&'a S) -> T) {
        assert!(self.depth != 0);
        self.column += 1;
    }
}

struct RowWriter<'a, S> {
    source: &'a S,
    row: RowBuf<'a>,
}
impl<'a, S> RowWrite<'a, S> for RowWriter<'a, S> {
    fn group(&mut self, _header: impl CellSource, f: impl FnOnce(&mut Self)) {
        f(self);
    }
    fn content<T: CellSource>(&mut self, f: impl FnOnce(&'a S) -> T) {
        self.row.push(f(self.source));
    }
}
