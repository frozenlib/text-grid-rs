use crate::cell::*;
use crate::grid_buf::*;
use std::cmp::max;
use std::fmt::*;
use std::marker::PhantomData;

pub trait RowWrite<'a, S: 'a + ?Sized> {
    fn group(&mut self, header: impl CellSource, f: impl FnOnce(&mut Self));
    fn content<T: CellSource>(&mut self, f: impl FnOnce(&'a S) -> T);

    fn column<T: CellSource>(&mut self, header: impl CellSource, f: impl FnOnce(&'a S) -> T) {
        self.group(header, |s| s.content(f));
    }
}

pub trait RowSource {
    fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>)
    where
        Self: 'a;
}

pub struct Grid<S> {
    buf: GridBuf,
    _phantom: PhantomData<Fn(&S)>,
}

impl<T: RowSource> Grid<T> {
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
    pub fn push_separator(&mut self) {
        self.buf.push_separator();
    }

    pub fn push_row(&mut self, source: &T) {
        let mut writer = RowWriter {
            source,
            row: self.buf.push_row(),
        };
        T::fmt_row(&mut writer);
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
    row: RowGuard<'a>,
    depth: usize,
    target: usize,
    column: usize,
    column_last: usize,
}
impl<'a> HeaderWriter<'a> {
    fn new(row: RowGuard<'a>, target: usize) -> Self {
        HeaderWriter {
            row,
            depth: 0,
            target,
            column: 0,
            column_last: 0,
        }
    }
    fn push_cell(&mut self, cell: impl CellSource) {
        if self.depth <= self.target {
            let colspan = self.column - self.column_last;
            self.row.push_with_colspan(cell, colspan);
            self.column_last = self.column;
        }
    }
}
impl<'a, S: 'a> RowWrite<'a, S> for HeaderWriter<'a> {
    fn group(&mut self, header: impl CellSource, f: impl FnOnce(&mut Self)) {
        self.push_cell(Cell::empty());
        self.depth += 1;
        f(self);
        self.depth -= 1;

        let mut style = CellStyle::default();
        style.h_align = Some(HorizontalAlignment::Center);

        let header = Cell::new(header).with_base_style(style);
        self.push_cell(header);
    }
    fn content<T: CellSource>(&mut self, _f: impl FnOnce(&'a S) -> T) {
        assert!(self.depth != 0);
        self.column += 1;
    }
}

struct RowWriter<'a, S> {
    source: &'a S,
    row: RowGuard<'a>,
}
impl<'a, S> RowWrite<'a, S> for RowWriter<'a, S> {
    fn group(&mut self, _header: impl CellSource, f: impl FnOnce(&mut Self)) {
        f(self);
    }
    fn content<T: CellSource>(&mut self, f: impl FnOnce(&'a S) -> T) {
        self.row.push(f(self.source));
    }
}
