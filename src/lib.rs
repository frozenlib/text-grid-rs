use std::cmp::*;
use std::fmt::*;
use unicode_width::UnicodeWidthStr;

pub struct TextGrid {
    b: String,
    cells: Vec<Cell>,
    rows: Vec<Row>,
}
pub struct TextGridRowGuard<'a>(&'a mut TextGrid);

struct Cell {
    b_end: usize,
    width: usize,
    align: Alignment,
}
enum Row {
    Cells { cells_end: usize },
    Separator,
}

impl TextGrid {
    pub fn new() -> Self {
        TextGrid {
            b: String::new(),
            cells: Vec::new(),
            rows: Vec::new(),
        }
    }
    pub fn row(&mut self) -> TextGridRowGuard {
        TextGridRowGuard(self)
    }
    fn row_end(&mut self) {
        self.rows.push(Row::Cells {
            cells_end: self.cells.len(),
        });
    }
    pub fn row_separator(&mut self) {
        self.rows.push(Row::Separator);
    }

    fn push_cell_by(&mut self, f: impl FnOnce(&mut String) -> Result, align: Alignment) -> Result {
        let idx = self.b.len();
        let r = f(&mut self.b);
        if r.is_ok() {
            self.cells.push(Cell {
                b_end: self.b.len(),
                width: self.b[idx..].width(),
                align,
            });
        } else {
            self.b.truncate(idx);
        }
        r
    }
}

impl TextGridRowGuard<'_> {
    pub fn cell(
        &mut self,
        value: impl Display,
        align: Alignment,
    ) -> std::result::Result<&mut Self, Error> {
        self.cell_by(|b| write!(b, "{}", value), align)
    }
    pub fn cell_by(
        &mut self,
        f: impl FnOnce(&mut String) -> Result,
        align: Alignment,
    ) -> std::result::Result<&mut Self, Error> {
        self.0.push_cell_by(f, align)?;
        Ok(self)
    }
}
impl Drop for TextGridRowGuard<'_> {
    fn drop(&mut self) {
        self.0.row_end()
    }
}

impl Display for TextGrid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut columns = Vec::new();
        let mut cell = 0;
        for row in &self.rows {
            if let &Row::Cells { cells_end } = row {
                let mut column = 0;
                while cell < cells_end {
                    if column == columns.len() {
                        columns.push(0);
                    }
                    columns[column] = max(columns[column], self.cells[cell].width);
                    column += 1;
                    cell += 1;
                }
            }
        }
        let mut b = 0;
        let mut cell = 0;
        for row in &self.rows {
            match row {
                &Row::Cells { cells_end } => {
                    let mut column = 0;
                    while cell < cells_end {
                        let width = columns[column];
                        let c = &self.cells[cell];
                        let s = &self.b[b..c.b_end];
                        write!(f, " ")?;
                        match c.align {
                            Alignment::Left => {
                                write!(f, "{0}{1:<p$}", s, "", p = width - c.width)?;
                            }
                            Alignment::Right => {
                                write!(f, "{1:<p$}{0}", s, "", p = width - c.width)?;
                            }
                            Alignment::Center => {
                                let p = width - c.width;
                                let lp = p / 2;
                                let rp = p - lp;
                                write!(f, "{1:<lp$}{0}{1:<rp$}", s, "", lp = lp, rp = rp)?;
                            }
                        }
                        write!(f, " |")?;
                        b = c.b_end;
                        column += 1;
                        cell += 1;
                    }
                }
                &Row::Separator => {
                    for column in &columns {
                        write!(f, "-{:-<w$}-|", "", w = column)?;
                    }
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
