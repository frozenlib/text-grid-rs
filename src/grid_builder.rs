use self::HorizontalAlignment::*;
use crate::cell::*;
use std::cmp::*;
use std::fmt::*;
use std::ops::Deref;
use unicode_width::UnicodeWidthStr;

/// A data structure that can be formatted into cells.
///
/// The number of columns must be statically determined from the type.
///
/// If the number of columns is dynamically determined, [`GridSchema`] must be used. See [`grid_schema`] for details.
pub trait CellsSource {
    /// Define columns. see [`CellsFormatter`] for details.
    fn fmt(f: &mut CellsFormatter<&Self>);
}
impl CellsSource for () {
    fn fmt(_: &mut CellsFormatter<&Self>) {}
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
            f.column(i, |x| &x[i]);
        }
    }
}
impl<T: CellsSource> CellsSource for Option<T> {
    fn fmt(f: &mut CellsFormatter<&Self>) {
        f.filter_map(|x| x.as_ref()).content(|x| *x)
    }
}

macro_rules! impl_cells_source_for_tuple {
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
impl_cells_source_for_tuple!(0: T0,);
impl_cells_source_for_tuple!(0: T0, 1: T1,);
impl_cells_source_for_tuple!(0: T0, 1: T1, 2: T2,);
impl_cells_source_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3,);
impl_cells_source_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4,);
impl_cells_source_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5,);
impl_cells_source_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6,);
impl_cells_source_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7,);
impl_cells_source_for_tuple!(
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
impl_cells_source_for_tuple!(
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
impl_cells_source_for_tuple!(
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
impl_cells_source_for_tuple!(
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
impl_cells_source_for_tuple!(
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
impl_cells_source_for_tuple!(
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
impl_cells_source_for_tuple!(
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
impl_cells_source_for_tuple!(
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

/// Column definitions.
///
/// Define columns using [`CellsFormatter`].
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
/// g.push(&[1, 2, 3]);
/// g.push(&[4, 5, 6]);
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

impl<R: ?Sized, T: GridSchema<R>> GridSchema<R> for Vec<T> {
    fn fmt(&self, f: &mut CellsFormatter<&R>) {
        for s in self {
            s.fmt(f);
        }
    }
}
impl<R: ?Sized, T: GridSchema<R>> GridSchema<R> for [T] {
    fn fmt(&self, f: &mut CellsFormatter<&R>) {
        for s in self {
            s.fmt(f);
        }
    }
}
impl<R: ?Sized, T: ?Sized + GridSchema<R>> GridSchema<R> for &T {
    fn fmt(&self, f: &mut CellsFormatter<&R>) {
        T::fmt(self, f)
    }
}

/// [`GridSchema`] implementation that use [`CellsSource`].
pub struct DefaultGridSchema<T: ?Sized>(std::marker::PhantomData<T>);

impl<T: CellsSource + ?Sized> Default for DefaultGridSchema<T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<T: CellsSource + ?Sized> GridSchema<T> for DefaultGridSchema<T> {
    fn fmt(&self, f: &mut CellsFormatter<&T>) {
        T::fmt(f);
    }
}

/// Create [`GridSchema`] from closure.
///
/// # Examples
///
/// By calculating the number of columns at runtime and creating a schema,
/// it is possible to create tables where the number of columns cannot be obtained statically.
///
/// ```rust
/// use text_grid::*;
/// let rows = vec![vec![1, 2, 3], vec![1, 2], vec![1, 2, 3, 4]];
/// let max_colunm_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);
/// let schema = grid_schema::<Vec<u32>>(move |f| {
///     for i in 0..max_colunm_count {
///         f.column(i, |x| x.get(i));
///     }
/// });
/// let mut g = Grid::new_with_schema(schema);
/// g.extend(rows);
/// assert_eq!(format!("\n{g}"), OUTPUT);
///
/// const OUTPUT: &str = r"
///  0 | 1 | 2 | 3 |
/// ---|---|---|---|
///  1 | 2 | 3 |   |
///  1 | 2 |   |   |
///  1 | 2 | 3 | 4 |
/// ";
/// ```
pub fn grid_schema<T>(fmt: impl Fn(&mut CellsFormatter<&T>)) -> impl GridSchema<T> {
    struct FnGridSchema<F>(F);
    impl<T, F: Fn(&mut CellsFormatter<&T>)> GridSchema<T> for FnGridSchema<F> {
        fn fmt(&self, f: &mut CellsFormatter<&T>) {
            (self.0)(f)
        }
    }
    FnGridSchema(fmt)
}

/// Used to define columns.
///
/// - Use [`column`](Self::column) to create column.
/// - Use [`group`](Self::group) to create multi level header.
/// - Use [`content`](Self::content) to create shared header columns.
pub struct CellsFormatter<'a, T> {
    w: &'a mut dyn CellsWrite,
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
    /// g.push(&RowData {
    ///     a: 300,
    ///     b_1: 10,
    ///     b_2: 20,
    /// });
    /// g.push(&RowData {
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
    /// - f : A function to obtain cells.
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
    /// g.push(&RowData {
    ///     a: 300,
    ///     b_1: 10,
    ///     b_2: 20,
    /// });
    /// g.push(&RowData {
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

    /// Define column content.
    ///
    /// - f : A function to obtain cell.
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
    /// g.push(&RowData { a: 300, b: 1 });
    /// g.push(&RowData { a: 2, b: 200 });
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

trait CellsWrite {
    fn content(&mut self, cell: Option<&dyn CellSource>);
    fn group_start(&mut self);
    fn group_end(&mut self, header: &dyn CellSource);
}

pub(crate) struct GridLayout {
    pub depth: usize,
    pub depth_max: usize,
    pub separators: Vec<bool>,
}
impl GridLayout {
    pub fn from_schema<T: ?Sized>(schema: &dyn GridSchema<T>) -> Self {
        let mut this = GridLayout::new();
        schema.fmt(&mut CellsFormatter {
            w: &mut this,
            d: None,
        });
        this.separators.pop();
        this
    }
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
impl CellsWrite for GridLayout {
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
impl CellsWrite for HeaderWriter<'_, '_> {
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

impl CellsWrite for BodyWriter<'_, '_> {
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

/// A builder used to create plain-text table.
///
/// # Examples
/// ```rust
/// use text_grid::*;
/// let mut g = GridBuilder::new();
/// g.push(|b| {
///     b.push(cell("name").right());
///     b.push("type");
///     b.push("value");
/// });
/// g.push_separator();
/// g.push(|b| {
///     b.push(cell(String::from("X")).right());
///     b.push("A");
///     b.push(10);
/// });
/// g.push(|b| {
///     b.push(cell("Y").right());
///     b.push_with_colspan(cell("BBB").center(), 2);
/// });
/// assert_eq!(format!("\n{g}"), r#"
///  name | type | value |
/// ------|------|-------|
///     X | A    |    10 |
///     Y |     BBB      |
/// "#);
/// ```
#[derive(Default)]
pub struct GridBuilder {
    s: String,
    cells: Vec<CellEntry>,
    rows: Vec<RowEntry>,
    columns: usize,
    column_separators: Vec<bool>,
}

struct CellEntry {
    s_idx: usize,
    width: usize,
    colspan: usize,
    style: CellStyle,
}
struct RowEntry {
    cells_idx: usize,
    has_separator: bool,
}

impl GridBuilder {
    /// Create a new `GridBuilder`.
    pub fn new() -> Self {
        GridBuilder {
            s: String::new(),
            cells: Vec::new(),
            rows: Vec::new(),
            columns: 0,
            column_separators: Vec::new(),
        }
    }

    pub(crate) fn new_with_header<T: ?Sized>(schema: &dyn GridSchema<T>) -> Self {
        let mut this = Self::new();
        let layout = GridLayout::from_schema(schema);
        this.set_column_separators(layout.separators);
        for target in 0..layout.depth_max {
            this.push(|b| {
                schema.fmt(&mut CellsFormatter {
                    w: &mut HeaderWriter::new(b, target),
                    d: None,
                })
            });
            this.push_separator();
        }
        this
    }

    /// Set column separator's visibility.
    ///
    /// `separators[0]` indicate visibility of the right side of the leftmost column.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use text_grid::*;
    /// let mut g = GridBuilder::new();
    /// g.push(|b| {
    ///     b.push("A");
    ///     b.push("B");
    ///     b.push("C");
    /// });
    /// g.push(|b| {
    ///     b.push("AAA");
    ///     b.push("BBB");
    ///     b.push("CCC");
    /// });
    /// g.set_column_separators(vec![true, true]);
    /// assert_eq!(format!("\n{g}"), r#"
    ///  A   | B   | C   |
    ///  AAA | BBB | CCC |
    /// "#);
    ///
    /// g.set_column_separators(vec![false, true]);
    /// assert_eq!(format!("\n{g}"), r#"
    ///  A  B   | C   |
    ///  AAABBB | CCC |
    /// "#);
    pub fn set_column_separators(&mut self, separators: Vec<bool>) {
        self.column_separators = separators;
    }

    /// Append a row to the bottom of the grid.
    pub fn push(&mut self, f: impl FnOnce(&mut RowBuilder)) {
        let cells_idx = self.cells.len();
        f(&mut RowBuilder {
            grid: self,
            cells_idx,
        })
    }

    /// Append a row separator to the bottom of the grid.
    pub fn push_separator(&mut self) {
        if let Some(row) = self.rows.last_mut() {
            row.has_separator = true;
        }
    }

    fn push_cell<S: CellSource>(&mut self, cell: S, colspan: usize) {
        let s_idx = self.s.len();
        cell.fmt(&mut self.s);
        self.cells.push(CellEntry {
            s_idx,
            width: self.s[s_idx..].width(),
            colspan,
            style: cell.style().or(cell.style_for_body()),
        });
    }
    fn get_width(&self, widths: &[usize], column: usize, colspan: usize) -> usize {
        assert!(colspan >= 1);
        let mut result = widths[column];
        for i in 1..colspan {
            if self.has_border(column + i) {
                result += 3;
            }
            result += widths[column + i];
        }
        result
    }
    fn has_border(&self, n: usize) -> bool {
        if n == 0 {
            false
        } else if n == self.columns {
            true
        } else if let Some(&value) = self.column_separators.get(n - 1) {
            value
        } else {
            true
        }
    }
    fn has_left_padding(&self, n: usize) -> bool {
        if n == 0 {
            true
        } else {
            self.has_border(n)
        }
    }
    fn has_right_padding(&self, n: usize) -> bool {
        if n == self.columns {
            true
        } else {
            self.has_border(n + 1)
        }
    }

    fn get_widths(&self) -> Vec<usize> {
        let mut widths = vec![0; self.columns];
        for row in 0..self.rows.len() {
            for c in self.row(row) {
                if c.colspan == 1 {
                    widths[c.column] = max(widths[c.column], c.width);
                }
            }
        }
        let mut smalls = Vec::new();
        for row in 0..self.rows.len() {
            for c in self.row(row) {
                if c.colspan > 1 {
                    let mut width_sum = self.get_width(&widths, c.column, c.colspan);
                    while width_sum < c.width {
                        smalls.clear();
                        smalls.push(0);
                        let mut min_width = widths[c.column];
                        let mut next_width = usize::max_value();
                        for i in 1..c.colspan {
                            let width = widths[c.column + i];
                            if width < min_width {
                                smalls.clear();
                                next_width = min_width;
                                min_width = width;
                            }
                            if width == min_width {
                                smalls.push(i);
                            }
                        }
                        for i in 0..smalls.len() {
                            let count = smalls.len() - i;
                            let expand_width_all = c.width - width_sum;
                            let expand_width = (expand_width_all + count - 1) / count;
                            let expand_width = min(expand_width, next_width - min_width);
                            width_sum += expand_width;
                            widths[c.column + smalls[i]] += expand_width;
                        }
                    }
                }
            }
        }
        widths
    }
    fn row(&self, row: usize) -> Cursor {
        Cursor {
            grid: self,
            column: 0,
            idx: self.cells_idx(row),
            end: self.cells_idx(row + 1),
        }
    }
    fn cells_idx(&self, row: usize) -> usize {
        if let Some(row) = self.rows.get(row) {
            row.cells_idx
        } else {
            self.cells.len()
        }
    }
    fn s_idx(&self, cells_idx: usize) -> usize {
        if let Some(cell) = self.cells.get(cells_idx) {
            cell.s_idx
        } else {
            self.s.len()
        }
    }
}

impl Display for GridBuilder {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let widths = self.get_widths();
        for row in 0..self.rows.len() {
            if self.has_border(0) {
                write!(f, "|")?;
            }
            for c in self.row(row) {
                let width = self.get_width(&widths, c.column, c.colspan);
                if self.has_left_padding(c.column) {
                    write!(f, " ")?;
                }
                let p = width - c.width;
                match c.style.align_h.unwrap_or(Left) {
                    Left => write!(f, "{0}{1:<p$}", c.s, "", p = p),
                    Right => write!(f, "{1:<p$}{0}", c.s, "", p = p),
                    Center => {
                        let lp = p / 2;
                        let rp = p - lp;
                        write!(f, "{1:<lp$}{0}{1:<rp$}", c.s, "", lp = lp, rp = rp)
                    }
                }?;
                if self.has_right_padding(c.column + c.colspan - 1) {
                    write!(f, " ")?;
                }
                if self.has_border(c.column + c.colspan) {
                    write!(f, "|")?;
                }
            }
            writeln!(f)?;
            if self.rows[row].has_separator {
                let mut cs = [self.row(row), self.row(row + 1)];
                for (column, _) in widths.iter().enumerate() {
                    if self.has_left_padding(column) {
                        write!(f, "-")?;
                    }
                    write!(f, "{:-<f$}", "", f = widths[column])?;
                    if self.has_right_padding(column) {
                        write!(f, "-")?;
                    }
                    for c in cs.iter_mut() {
                        while c.column <= column && c.next().is_some() {}
                    }
                    if self.has_border(column + 1) {
                        if cs.iter().all(|x| x.column == column + 1) {
                            write!(f, "|")?;
                        } else {
                            write!(f, "-")?;
                        }
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
impl Debug for GridBuilder {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

/// A builder used to create row of [`GridBuilder`].
///
/// This structure is created by [`GridBuilder::push`].
pub struct RowBuilder<'a> {
    grid: &'a mut GridBuilder,
    cells_idx: usize,
}

impl RowBuilder<'_> {
    /// Append a cell to the right of row.
    pub fn push(&mut self, cell: impl CellSource) {
        self.grid.push_cell(cell, 1);
    }

    /// Append a multi-column cell to the right of row.
    ///
    /// - `cell` : Contents of cell to be appended.
    /// - `colspan` : Number of columns used by the cell to be appended.
    ///
    /// if `colspan == 0`, this method will do nothing.
    pub fn push_with_colspan(&mut self, cell: impl CellSource, colspan: usize) {
        if colspan != 0 {
            self.grid.push_cell(cell, colspan);
        }
    }

    pub fn extend<T: ?Sized + CellsSource>(&mut self, source: &T) {
        T::fmt(&mut CellsFormatter {
            w: &mut BodyWriter(self),
            d: Some(source),
        })
    }
    pub fn extend_with_schema<T: ?Sized>(&mut self, source: &T, schema: &dyn GridSchema<T>) {
        schema.fmt(&mut CellsFormatter {
            w: &mut BodyWriter(self),
            d: Some(source),
        })
    }
}
impl Drop for RowBuilder<'_> {
    fn drop(&mut self) {
        self.grid.columns = max(self.grid.columns, self.grid.cells.len() - self.cells_idx);
        self.grid.rows.push(RowEntry {
            cells_idx: self.cells_idx,
            has_separator: false,
        });
    }
}

struct Cursor<'a> {
    grid: &'a GridBuilder,
    column: usize,
    idx: usize,
    end: usize,
}
impl<'a> Iterator for Cursor<'a> {
    type Item = CellRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.end {
            None
        } else {
            let g = self.grid;
            let r = CellRef {
                cell: &g.cells[self.idx],
                s: &g.s[g.s_idx(self.idx)..g.s_idx(self.idx + 1)],
                column: self.column,
            };
            self.column += r.colspan;
            self.idx += 1;
            Some(r)
        }
    }
}

struct CellRef<'a> {
    cell: &'a CellEntry,
    s: &'a str,
    column: usize,
}
impl<'a> Deref for CellRef<'a> {
    type Target = &'a CellEntry;
    fn deref(&self) -> &Self::Target {
        &self.cell
    }
}
