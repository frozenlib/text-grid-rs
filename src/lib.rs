use std::cmp::*;
use std::fmt::Alignment::*;
use std::fmt::*;
use std::ops::Deref;
use unicode_width::UnicodeWidthStr;

pub struct TextGrid {
    s: String,
    cells: Vec<Cell>,
    rows: Vec<Row>,
}
pub struct TextGridRowGuard<'a> {
    grid: &'a mut TextGrid,
    cells_idx: usize,
}

struct Cell {
    s_idx: usize,
    width: usize,
    colspan: usize,
    align: Alignment,
}
struct Row {
    cells_idx: usize,
    has_separator: bool,
}

impl TextGrid {
    pub fn new() -> Self {
        TextGrid {
            s: String::new(),
            cells: Vec::new(),
            rows: Vec::new(),
        }
    }
    pub fn push_row(&mut self) -> TextGridRowGuard {
        let cells_idx = self.cells.len();
        TextGridRowGuard {
            grid: self,
            cells_idx,
        }
    }
    pub fn push_separator(&mut self) {
        if let Some(row) = self.rows.last_mut() {
            row.has_separator = true;
        }
    }

    fn push_cell_by(
        &mut self,
        f: impl FnOnce(&mut String) -> Result,
        colspan: usize,
        align: Alignment,
    ) -> Result {
        let s_idx = self.s.len();
        let r = f(&mut self.s);
        if r.is_ok() {
            self.cells.push(Cell {
                s_idx,
                width: self.s[s_idx..].width(),
                colspan,
                align,
            });
        } else {
            self.s.truncate(s_idx);
        }
        r
    }
}

impl TextGridRowGuard<'_> {
    pub fn push(
        &mut self,
        value: impl Display,
        align: Alignment,
    ) -> std::result::Result<&mut Self, Error> {
        self.push_by(|b| write!(b, "{}", value), align)
    }
    pub fn push_by(
        &mut self,
        f: impl FnOnce(&mut String) -> Result,
        align: Alignment,
    ) -> std::result::Result<&mut Self, Error> {
        self.grid.push_cell_by(f, 1, align)?;
        Ok(self)
    }
    pub fn push_merged(&mut self, mut count: usize) -> &mut Self {
        if count != 0 {
            if !self.has_cells() {
                self.push_empty();
                count -= 1;
            }
            self.grid.cells.last_mut().unwrap().colspan += count;
        }
        self
    }
    fn has_cells(&self) -> bool {
        self.cells_idx < self.grid.cells.len()
    }

    pub fn push_empty(&mut self) -> &mut Self {
        self.grid.cells.push(Cell {
            s_idx: self.grid.s.len(),
            width: 0,
            colspan: 1,
            align: Alignment::Left,
        });
        self
    }
}
impl Drop for TextGridRowGuard<'_> {
    fn drop(&mut self) {
        self.grid.rows.push(Row {
            cells_idx: self.cells_idx,
            has_separator: false,
        });
    }
}

impl TextGrid {
    fn get_widths(&self) -> Vec<usize> {
        let mut widths = Vec::new();
        for row in 0..self.rows.len() {
            for c in self.row(row) {
                if widths.len() <= c.column {
                    widths.push(0);
                }
                if c.colspan == 1 {
                    widths[c.column] = max(widths[c.column], c.width);
                }
            }
        }
        let mut smalls = Vec::new();
        for row in 0..self.rows.len() {
            for c in self.row(row) {
                if c.colspan > 1 {
                    let colspan = min(widths.len() - c.column, c.colspan);
                    let mut width_sum: usize = widths[c.column..c.column + colspan].iter().sum();
                    width_sum = width_sum + (colspan - 1) * 3;
                    while width_sum < c.width {
                        smalls.clear();
                        smalls.push(0);
                        let mut min_width = widths[c.column];
                        let mut next_width = usize::max_value();
                        for i in 1..colspan {
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

impl Display for TextGrid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let widths = self.get_widths();
        for row in 0..self.rows.len() {
            for c in self.row(row) {
                let colspan = min(widths.len() - c.column, c.colspan);
                let width: usize = widths[c.column..c.column + colspan].iter().sum();
                let width = width + (colspan - 1) * 3;
                write!(f, " ")?;
                let p = width - c.width;
                match c.align {
                    Left => write!(f, "{0}{1:<p$}", c.s, "", p = p),
                    Right => write!(f, "{1:<p$}{0}", c.s, "", p = p),
                    Center => {
                        let lp = p / 2;
                        let rp = p - lp;
                        write!(f, "{1:<lp$}{0}{1:<rp$}", c.s, "", lp = lp, rp = rp)
                    }
                }?;
                write!(f, " |")?;
            }
            writeln!(f, "")?;
            if self.rows[row].has_separator {
                let mut cs = [self.row(row), self.row(row + 1)];
                for column in 0..widths.len() {
                    let width = widths[column];
                    write!(f, "-{:-<w$}-", "", w = width)?;
                    for c in cs.iter_mut() {
                        while c.column <= column && c.next().is_some() {}
                    }
                    if cs.iter().all(|x| x.column == column + 1) {
                        write!(f, "|")?;
                    } else {
                        write!(f, "-")?;
                    }
                }
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}
struct Cursor<'a> {
    grid: &'a TextGrid,
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
            self.column += g.cells[self.idx].colspan;
            self.idx += 1;
            Some(r)
        }
    }
}

struct CellRef<'a> {
    cell: &'a Cell,
    s: &'a str,
    column: usize,
}
impl<'a> Deref for CellRef<'a> {
    type Target = &'a Cell;
    fn deref(&self) -> &Self::Target {
        &self.cell
    }
}
