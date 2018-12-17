use self::HorizontalAlignment::*;
use crate::cell::*;
use std::cmp::*;
use std::fmt::*;
use std::ops::Deref;
use unicode_width::UnicodeWidthStr;

/// A builder used to create plain-text table.
///
/// # Examples
/// ```rust
/// use text_grid::*;
/// let mut g = GridBuf::new();
/// {
///     let mut row = g.push_row();
///     row.push(cell("name").right());
///     row.push("type");
///     row.push("value");
/// }
/// g.push_separator();
/// {
///     let mut row = g.push_row();
///     row.push(cell(String::from("X")).right());
///     row.push("A");
///     row.push(10);
/// }
/// {
///     let mut row = g.push_row();
///     row.push(cell("Y").right());
///     row.push_with_colspan(cell("BBB").center(), 2);
/// }
///
/// print!("{}", g);
/// ```
///
/// Output:
/// ```text
///  name | type | value |
/// ------|------|-------|
///     X | A    |    10 |
///     Y |     BBB      |
/// ```
pub struct GridBuf {
    s: String,
    cells: Vec<CellEntry>,
    rows: Vec<RowEntry>,
    columns: usize,
    column_separators: Vec<bool>,
}

/// A builder used to create row of [`GridBuf`].
///
/// This structure is created by [`GridBuf::push_row`].
pub struct RowBuf<'a> {
    grid: &'a mut GridBuf,
    cells_idx: usize,
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

impl GridBuf {
    /// Create a new `GridBuf`.
    pub fn new() -> Self {
        GridBuf {
            s: String::new(),
            cells: Vec::new(),
            rows: Vec::new(),
            columns: 0,
            column_separators: Vec::new(),
        }
    }

    /// Set column separator's visibility.
    ///
    /// `separators[0]` indicate visibility of the right side of the leftmost column.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use text_grid::*;
    /// let mut g = GridBuf::new();
    /// {
    ///     let mut row = g.push_row();
    ///     row.push("A");
    ///     row.push("B");
    ///     row.push("C");
    /// }
    /// {
    ///     let mut row = g.push_row();
    ///     row.push("AAA");
    ///     row.push("BBB");
    ///     row.push("CCC");
    /// }
    /// g.set_column_separators(vec![true, true]);
    /// println!("{:?}", vec![true, true]);
    /// println!("{}", g);
    ///
    /// g.set_column_separators(vec![false, true]);
    /// println!("{:?}", vec![false, true]);
    /// println!("{}", g);
    ///
    /// ```
    ///
    /// Output:
    /// ```txt
    /// [true, true]
    ///  A   | B   | C   |
    ///  AAA | BBB | CCC |
    ///
    /// [false, true]
    ///  A  B   | C   |
    ///  AAABBB | CCC |  
    /// ```
    pub fn set_column_separators(&mut self, separators: Vec<bool>) {
        self.column_separators = separators;
    }

    /// Append a row to the bottom of the grid.
    pub fn push_row(&mut self) -> RowBuf {
        let cells_idx = self.cells.len();
        RowBuf {
            grid: self,
            cells_idx,
        }
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
            style: cell.style().or(S::default_style()),
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

impl Display for GridBuf {
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
            writeln!(f, "")?;
            if self.rows[row].has_separator {
                let mut cs = [self.row(row), self.row(row + 1)];
                for column in 0..widths.len() {
                    if self.has_left_padding(column) {
                        write!(f, "-")?;
                    }
                    write!(f, "{:-<w$}", "", w = widths[column])?;
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
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

impl RowBuf<'_> {
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
}
impl Drop for RowBuf<'_> {
    fn drop(&mut self) {
        self.grid.columns = max(self.grid.columns, self.grid.cells.len() - self.cells_idx);
        self.grid.rows.push(RowEntry {
            cells_idx: self.cells_idx,
            has_separator: false,
        });
    }
}

struct Cursor<'a> {
    grid: &'a GridBuf,
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
