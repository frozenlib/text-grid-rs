use std::cmp::*;
use std::fmt::*;
use unicode_width::UnicodeWidthStr;

pub struct TextGrid {
    b: String,
    cells: Vec<Cell>,
    rows: Vec<Row>,
}
struct Cell {
    b_end: usize,
    width: usize,
    align: Alignment,
}
struct Row {
    cells_end: usize,
    with_separator: bool,
}

impl TextGrid {
    pub fn new() -> Self {
        TextGrid {
            b: String::new(),
            cells: Vec::new(),
            rows: Vec::new(),
        }
    }
    pub fn push_cell(&mut self, value: impl Display, align: Alignment) -> Result {
        self.push_cell_by(|b| write!(b, "{}", value), align)
    }
    pub fn push_cell_by(
        &mut self,
        f: impl FnOnce(&mut String) -> Result,
        align: Alignment,
    ) -> Result {
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

    pub fn finish_row(&mut self) {
        self.finish_row_with(false)
    }
    pub fn finish_row_with(&mut self, with_separator: bool) {
        self.rows.push(Row {
            cells_end: self.cells.len(),
            with_separator,
        });
    }
}
impl Display for TextGrid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut columns = Vec::new();
        let mut cell = 0;
        for row in &self.rows {
            let mut column = 0;
            while cell < row.cells_end {
                if column == columns.len() {
                    columns.push(0);
                }
                columns[column] = max(columns[column], self.cells[cell].width);
                column += 1;
                cell += 1;
            }
        }
        let mut b = 0;
        let mut cell = 0;
        for row in &self.rows {
            let mut idx_column = 0;
            while cell < row.cells_end {
                let width = columns[idx_column];
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
                idx_column += 1;
                cell += 1;
            }
            writeln!(f, "")?;
            if row.with_separator {
                for column in &columns {
                    write!(f, "-{:-<w$}-|", "", w = column)?;
                }
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}
