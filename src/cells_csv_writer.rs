use std::{borrow::Borrow, io::Write};

use csv::{StringRecord, Writer};

use crate::{CellsFormatter, CellsSchema, CellsWrite, RawCell};

pub fn write_csv<T>(
    csv_writer: &mut Writer<impl Write>,
    source: impl IntoIterator<Item = impl Borrow<T>>,
    schema: impl CellsSchema<Source = T>,
    separator: &str,
) -> csv::Result<()> {
    let source = source.into_iter();
    let mut w = CsvHeaderWriter::new(separator);
    schema.fmt(&mut CellsFormatter::new(&mut w, None));
    csv_writer.write_record(&w.record)?;

    let mut w = CsvBodyWriter::new();
    for item in source {
        schema.fmt(&mut CellsFormatter::new(&mut w, Some(item.borrow())));
        csv_writer.write_record(&w.record)?;
        w.record.clear();
    }
    Ok(())
}

struct CsvHeaderWriter<'a> {
    pub record: StringRecord,
    value: String,
    lens: Vec<usize>,
    has_content: bool,
    separator: &'a str,
}
impl<'a> CsvHeaderWriter<'a> {
    fn new(separator: &'a str) -> Self {
        Self {
            record: StringRecord::new(),
            value: String::new(),
            lens: Vec::new(),
            has_content: false,
            separator,
        }
    }
}

impl CellsWrite for CsvHeaderWriter<'_> {
    fn content(&mut self, _cell: Option<&dyn RawCell>, _stretch: bool) {
        self.has_content = true;
    }

    fn merged_body_start(&mut self, _cell: &dyn RawCell) {}
    fn merged_body_end(&mut self, _cell: &dyn RawCell) {}

    fn column_start(&mut self, header: &dyn RawCell) {
        self.lens.push(self.value.len());
        if !self.value.is_empty() {
            self.value.push_str(self.separator);
        }
        header.fmt(&mut self.value);
    }

    fn column_end(&mut self, _header: &dyn RawCell) {
        if self.has_content {
            self.record.push_field(&self.value);
            self.has_content = false;
        }
        self.value.truncate(self.lens.pop().unwrap());
    }
}

struct CsvBodyWriter {
    pub record: StringRecord,
    value: String,
    is_merged: bool,
    has_content: bool,
}

impl CsvBodyWriter {
    fn new() -> Self {
        Self {
            record: StringRecord::new(),
            value: String::new(),
            is_merged: false,
            has_content: false,
        }
    }
}

impl CellsWrite for CsvBodyWriter {
    fn content(&mut self, cell: Option<&dyn RawCell>, _stretch: bool) {
        if let Some(cell) = cell {
            cell.fmt(&mut self.value);
        }
        self.has_content = true;
    }

    fn merged_body_start(&mut self, cell: &dyn RawCell) {
        self.is_merged = true;
        cell.fmt(&mut self.value);
    }

    fn merged_body_end(&mut self, _cell: &dyn RawCell) {
        self.is_merged = false;
    }

    fn column_start(&mut self, _header: &dyn RawCell) {}

    fn column_end(&mut self, _header: &dyn RawCell) {
        if self.has_content {
            self.record.push_field(&self.value);
            self.value.clear();
            self.has_content = false;
        }
    }
}
