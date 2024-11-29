use self::HorizontalAlignment::*;
use crate::cell::*;
use crate::Cells;
use crate::CellsFormatter;
use crate::CellsSchema;
use crate::CellsWrite;
use crate::DefaultCellsSchema;
use derive_ex::derive_ex;
use std::borrow::Borrow;
use std::cmp::*;
use std::collections::HashMap;
use std::fmt::*;
use std::ops::Deref;
use unicode_width::UnicodeWidthStr;

struct GridLayout {
    depth: usize,
    depth_max: usize,
    styles: Vec<ColumnStyle>,
}
impl GridLayout {
    pub fn from_schema<T: ?Sized>(schema: &dyn CellsSchema<Source = T>) -> Self {
        let mut this = GridLayout::new();
        schema.fmt(&mut CellsFormatter::new(&mut this, None));
        this.styles.pop();
        this
    }
    fn new() -> Self {
        Self {
            depth: 0,
            depth_max: 0,
            styles: Vec::new(),
        }
    }
    fn set_column_end_style(&mut self) {
        if let Some(last) = self.styles.last_mut() {
            last.column_end = true;
        }
    }
}
impl CellsWrite for GridLayout {
    fn content(&mut self, _cell: Option<&dyn RawCell>, stretch: bool) {
        self.styles.push(ColumnStyle {
            column_end: false,
            stretch,
        });
    }
    fn merged_body_start(&mut self, _cell: &dyn RawCell) {}
    fn merged_body_end(&mut self, _cell: &dyn RawCell) {}
    fn column_start(&mut self, _header: &dyn RawCell) {
        self.set_column_end_style();
        self.depth += 1;
        self.depth_max = max(self.depth_max, self.depth);
    }

    fn column_end(&mut self, _header: &dyn RawCell) {
        self.depth -= 1;
        self.set_column_end_style()
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

    fn push_cell(&mut self, cell: impl RawCell) {
        let colspan = self.column - self.column_last;
        self.b.push_with_colspan(cell, colspan);
        self.column_last = self.column;
    }
}
impl CellsWrite for HeaderWriter<'_, '_> {
    fn content(&mut self, _cell: Option<&dyn RawCell>, _stretch: bool) {
        self.column += 1;
    }
    fn merged_body_start(&mut self, _cell: &dyn RawCell) {}
    fn merged_body_end(&mut self, _cell: &dyn RawCell) {}
    fn column_start(&mut self, _header: &dyn RawCell) {
        if self.depth <= self.target {
            self.push_cell(Cell::empty());
        }
        self.depth += 1;
    }
    fn column_end(&mut self, header: &dyn RawCell) {
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
struct BodyWriter<'a, 'b> {
    b: &'a mut RowBuilder<'b>,
    colspan: Option<usize>,
}

impl<'a, 'b> BodyWriter<'a, 'b> {
    fn new(b: &'a mut RowBuilder<'b>) -> Self {
        Self { b, colspan: None }
    }
}

impl CellsWrite for BodyWriter<'_, '_> {
    fn content(&mut self, cell: Option<&dyn RawCell>, _stretch: bool) {
        if let Some(colspan) = &mut self.colspan {
            *colspan += 1;
        } else {
            self.b.push(cell);
        }
    }
    fn merged_body_start(&mut self, _cell: &dyn RawCell) {
        assert!(self.colspan.is_none());
        self.colspan = Some(0);
    }
    fn merged_body_end(&mut self, cell: &dyn RawCell) {
        let colspan = self.colspan.take().unwrap();
        self.b.push_with_colspan(cell, colspan);
    }

    fn column_start(&mut self, _header: &dyn RawCell) {}
    fn column_end(&mut self, _header: &dyn RawCell) {}
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
#[derive_ex(Default)]
#[default(Self::new())]
pub struct GridBuilder {
    s: String,
    cells: Vec<CellEntry>,
    rows: Vec<RowEntry>,
    columns: usize,
    pub column_styles: Vec<ColumnStyle>,
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
            column_styles: Vec::new(),
        }
    }

    pub fn from_iter_with_schema<T>(
        source: impl IntoIterator<Item = impl Borrow<T>>,
        schema: impl CellsSchema<Source = T>,
    ) -> Self {
        let mut this = Self::new();
        this.extend_header_with_schema(&schema);
        this.extend_body_with_schema(source, &schema);
        this
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

    pub fn extend_header<T: ?Sized + Cells>(&mut self) {
        self.extend_header_with_schema::<T>(&DefaultCellsSchema::default());
    }

    pub fn extend_header_with_schema<T: ?Sized>(&mut self, schema: impl CellsSchema<Source = T>) {
        let layout = GridLayout::from_schema(&schema);
        self.column_styles = layout.styles;
        for target in 0..layout.depth_max {
            self.push(|b| {
                schema.fmt(&mut CellsFormatter::new(
                    &mut HeaderWriter::new(b, target),
                    None,
                ))
            });
            self.push_separator();
        }
    }

    pub fn push_body(&mut self, source: &impl Cells) {
        self.push_body_with_schema(source, DefaultCellsSchema::default());
    }
    pub fn push_body_with_schema<T: ?Sized>(
        &mut self,
        source: &T,
        schema: impl CellsSchema<Source = T>,
    ) {
        self.push(|b| {
            b.extend_with_schema(source, &schema);
        });
    }
    pub fn extend_body(&mut self, source: impl IntoIterator<Item = impl Cells>) {
        self.extend_body_with_schema(source, DefaultCellsSchema::default());
    }
    pub fn extend_body_with_schema<T>(
        &mut self,
        source: impl IntoIterator<Item = impl Borrow<T>>,
        schema: impl CellsSchema<Source = T>,
    ) {
        for source in source {
            self.push(|b| {
                b.extend_with_schema(source.borrow(), &schema);
            });
        }
    }

    fn push_cell<S: RawCell>(&mut self, cell: S, colspan: usize) {
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
        } else if n >= self.columns {
            true
        } else {
            self.column_style(n - 1).column_end
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

    fn column_style(&self, column: usize) -> &ColumnStyle {
        self.column_styles
            .get(column)
            .unwrap_or(&ColumnStyle::DEFAULT)
    }
    fn stretch_count(&self, column: usize, colspan: usize) -> usize {
        let mut count = 0;
        for i in 0..colspan {
            if self.column_style(column + i).stretch {
                count += 1;
            }
        }
        count
    }

    fn get_widths(&self) -> Vec<usize> {
        #[derive(PartialEq, Eq, Hash)]
        struct ColRange {
            colspan: usize,
            column: usize,
        }

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct Block {
            stretch: usize,
            colspan: usize,
            column: usize,
            width: usize,
        }

        let mut widths = vec![0; self.columns];
        let mut blocks = HashMap::new();
        for row in self.rows() {
            for c in row {
                let e = if c.colspan == 1 {
                    &mut widths[c.column]
                } else {
                    let key = ColRange {
                        colspan: c.colspan,
                        column: c.column,
                    };
                    blocks.entry(key).or_insert(0)
                };
                *e = max(*e, c.width);
            }
        }
        let mut blocks: Vec<_> = blocks
            .into_iter()
            .map(|c| Block {
                stretch: self.stretch_count(c.0.column, c.0.colspan),
                colspan: c.0.colspan,
                column: c.0.column,
                width: c.1,
            })
            .collect();
        blocks.sort();

        let mut expand_cols = Vec::new();
        for b in blocks {
            let mut width_sum = self.get_width(&widths, b.column, b.colspan);
            let start = if b.stretch == 0 {
                b.column
            } else {
                (b.column..b.column + b.colspan)
                    .find(|&column| self.column_style(column).stretch)
                    .unwrap()
            };

            while width_sum < b.width {
                expand_cols.clear();
                expand_cols.push(start);
                let mut min_width = widths[start];
                let mut next_width = usize::MAX;
                #[allow(clippy::needless_range_loop)]
                for column in start + 1..b.column + b.colspan {
                    if b.stretch == 0 || self.column_style(column).stretch {
                        let width = widths[column];
                        if width < min_width {
                            expand_cols.clear();
                            next_width = min_width;
                            min_width = width;
                        }
                        if width == min_width {
                            expand_cols.push(column);
                        }
                    }
                }
                for i in 0..expand_cols.len() {
                    let count = expand_cols.len() - i;
                    let expand_width_all = b.width - width_sum;
                    let expand_width = expand_width_all.div_ceil(count);
                    let expand_width = min(expand_width, next_width - min_width);
                    width_sum += expand_width;
                    widths[expand_cols[i]] += expand_width;
                }
            }
        }
        widths
    }
    fn row(&self, row: usize) -> Option<Cursor> {
        if row < self.rows.len() {
            Some(Cursor {
                grid: self,
                column: 0,
                idx: self.cells_idx(row),
                end: self.cells_idx(row + 1),
            })
        } else {
            None
        }
    }
    fn rows(&self) -> impl Iterator<Item = Cursor> {
        (0..self.rows.len()).map(|row| self.row(row).unwrap())
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
            for c in self.row(row).unwrap() {
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
                    for c in cs.iter_mut().flatten() {
                        while c.column <= column && c.next().is_some() {}
                    }
                    if self.has_border(column + 1) {
                        if cs.iter().flatten().all(|x| x.column == column + 1) {
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
impl<T: Cells> FromIterator<T> for GridBuilder {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut b = Self::new();
        b.extend_header::<T>();
        b.extend_body(iter);
        b
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
    pub fn push(&mut self, cell: impl RawCell) {
        self.grid.push_cell(cell, 1);
    }

    /// Append a multi-column cell to the right of row.
    ///
    /// - `cell` : Contents of cell to be appended.
    /// - `colspan` : Number of columns used by the cell to be appended.
    ///
    /// if `colspan == 0`, this method will do nothing.
    pub fn push_with_colspan(&mut self, cell: impl RawCell, colspan: usize) {
        if colspan != 0 {
            self.grid.push_cell(cell, colspan);
        }
    }

    pub fn extend<T: ?Sized + Cells>(&mut self, source: &T) {
        T::fmt(&mut CellsFormatter::new(
            &mut BodyWriter::new(self),
            Some(source),
        ))
    }
    pub fn extend_with_schema<T: ?Sized>(
        &mut self,
        source: &T,
        schema: impl CellsSchema<Source = T>,
    ) {
        schema.fmt(&mut CellsFormatter::new(
            &mut BodyWriter::new(self),
            Some(source),
        ))
    }
}
impl Drop for RowBuilder<'_> {
    fn drop(&mut self) {
        let mut columns = 0;
        for cell in &self.grid.cells[self.cells_idx..] {
            columns += cell.colspan;
        }
        self.grid.columns = max(self.grid.columns, columns);
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

/// Column's style.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive_ex(Default)]
#[default(Self::DEFAULT)]
pub struct ColumnStyle {
    /// If true, display a separator on the right side of this column.
    ///
    /// This setting is ignored for the rightmost column, and the border is always displayed.
    ///
    /// The default for this is `true`.
    ///
    /// ```
    /// use text_grid::*;
    /// let mut g = GridBuilder::new();
    /// g.push(|b| {
    ///     b.push("A");
    ///     b.push("B");
    ///     b.push("C");
    /// });
    /// assert_eq!(format!("\n{g}"), E0);
    ///
    /// g.column_styles = vec![ColumnStyle::default(); 2];
    /// g.column_styles[0].column_end = false;
    ///
    /// assert_eq!(format!("\n{g}"), E1);
    ///
    /// const E0: &str = r"
    ///  A | B | C |
    /// ";
    ///
    /// const E1: &str = r"
    ///  AB | C |
    /// ";
    /// ```
    pub column_end: bool,

    /// If true, prioritize this column width expansion over others.
    ///
    /// When stretching a multi-column layout,
    /// if any column has `stretch` set to true, only those columns will be stretched,
    /// while columns with `stretch` set to false will not be stretched.
    ///
    /// The default for this is `false`.
    ///
    /// ```
    /// use text_grid::*;
    /// let mut g = GridBuilder::new();
    /// g.push(|b| {
    ///     b.push_with_colspan("............", 2);
    /// });
    /// g.push(|b| {
    ///     b.push("A");
    ///     b.push("B");
    /// });
    /// assert_eq!(format!("\n{g}"), E0);
    ///
    /// g.column_styles = vec![ColumnStyle::default(); 2];
    /// g.column_styles[0].stretch = true;
    ///
    /// assert_eq!(format!("\n{g}"), E1);
    ///
    /// const E0: &str = r"
    ///  ............ |
    ///  A     | B    |
    /// ";
    ///
    /// const E1: &str = r"
    ///  ............ |
    ///  A        | B |
    /// ";
    /// ```
    pub stretch: bool,
}
impl ColumnStyle {
    const DEFAULT: Self = Self {
        column_end: true,
        stretch: false,
    };
}
