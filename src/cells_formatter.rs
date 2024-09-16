use crate::cell::*;
use crate::Cells;

/// Used to define columns.
///
/// - Use [`column`](Self::column) to create column.
/// - Use [`column_with`](Self::column_with) to create multi level header.
/// - Use [`content`](Self::content) to create shared header columns.
pub struct CellsFormatter<'a, 'b, T: ?Sized> {
    w: &'a mut dyn CellsWrite,
    d: Option<&'b T>,
    stretch: bool,
}

impl<'a, 'b, T: ?Sized> CellsFormatter<'a, 'b, T> {
    pub(crate) fn new(w: &'a mut dyn CellsWrite, d: Option<&'b T>) -> Self {
        Self {
            w,
            d,
            stretch: false,
        }
    }

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
    /// impl Cells for RowData {
    ///     fn fmt(f: &mut CellsFormatter<Self>) {
    ///         f.column("a", |s| s.a);
    ///         f.column_with("b", |f| {
    ///             f.column("1", |s| s.b_1);
    ///             f.column("2", |s| s.b_2);
    ///         });
    ///     }
    /// }
    ///
    /// let rows = [
    ///     RowData { a: 300,b_1: 10, b_2: 20 },
    ///     RowData { a: 300,b_1: 1, b_2: 500 },
    /// ];
    /// let g = to_grid(rows);
    /// assert_eq!(format!("\n{g}"), r#"
    ///   a  |    b     |
    /// -----|----------|
    ///      | 1  |  2  |
    /// -----|----|-----|
    ///  300 | 10 |  20 |
    ///  300 |  1 | 500 |
    /// "#);
    /// ```
    pub fn column_with(
        &mut self,
        header: impl RawCell,
        f: impl FnOnce(&mut CellsFormatter<'_, 'b, T>),
    ) {
        self.w.column_start(&header);
        f(self);
        self.w.column_end(&header);
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
    /// impl Cells for RowData {
    ///     fn fmt(f: &mut CellsFormatter<Self>) {
    ///         f.column("a", |s| s.a);
    ///         f.column_with("b", |f| {
    ///             f.content(|s| s.b_1);
    ///             f.content(|_| " ");
    ///             f.content(|s| s.b_2);
    ///         });
    ///     }
    /// }
    ///
    /// let rows = [
    ///     RowData { a: 300, b_1: 10, b_2: 20 },
    ///     RowData { a: 300, b_1: 1, b_2: 500 },
    /// ];
    /// let g = to_grid(rows);
    /// assert_eq!(format!("\n{g}"), r#"
    ///   a  |   b    |
    /// -----|--------|
    ///  300 | 10  20 |
    ///  300 |  1 500 |
    /// "#);
    /// ```
    pub fn content<U: Cells>(&mut self, f: impl FnOnce(&'b T) -> U) {
        self.map_with(f, U::fmt)
    }

    /// Define column content.
    ///
    /// - f : A function to obtain cell.
    pub(crate) fn content_cell<U: RawCell>(&mut self, f: impl FnOnce(&'b T) -> U) {
        self.w.content(
            self.d.map(f).as_ref().map(|x| x as &dyn RawCell),
            self.stretch,
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
    /// impl Cells for RowData {
    ///     fn fmt(f: &mut CellsFormatter<Self>) {
    ///         f.column("a", |s| s.a);
    ///         f.column("b", |s| s.b);
    ///     }
    /// }
    ///
    /// let rows = [
    ///     RowData { a: 300, b: 1 },
    ///     RowData { a: 2, b: 200 },
    /// ];
    /// let g = to_grid(rows);
    /// assert_eq!(format!("\n{g}"), r#"
    ///   a  |  b  |
    /// -----|-----|
    ///  300 |   1 |
    ///    2 | 200 |
    /// "#);
    /// ```
    pub fn column<U: Cells>(&mut self, header: impl RawCell, f: impl FnOnce(&'b T) -> U) {
        self.column_with(header, |cf| cf.content(f));
    }

    /// Creates a [`CellsFormatter`] whose source value was converted.
    ///
    /// If you want to convert to an owned value instead of a reference, use [`map_with`](Self::map_with) instead.
    pub fn map<U: ?Sized>(&mut self, m: impl FnOnce(&T) -> &U) -> CellsFormatter<'_, 'b, U> {
        CellsFormatter {
            w: self.w,
            d: self.d.map(m),
            stretch: self.stretch,
        }
    }

    /// Creates a [`CellsFormatter`] whose source value was converted.
    ///
    /// Unlike [`map`](Self::map), `m` can be converted to an owned value.
    pub fn map_with<U>(
        &mut self,
        m: impl FnOnce(&'b T) -> U,
        f: impl FnOnce(&mut CellsFormatter<U>),
    ) {
        f(&mut CellsFormatter {
            w: self.w,
            d: self.d.map(m).as_ref(),
            stretch: self.stretch,
        });
    }

    /// Creates a [`CellsFormatter`] that outputs the body cell only when the source value satisfies the condition.
    pub fn filter(&mut self, f: impl FnOnce(&T) -> bool) -> CellsFormatter<T> {
        CellsFormatter {
            w: self.w,
            d: self.d.filter(|data| f(data)),
            stretch: self.stretch,
        }
    }

    /// Creates a [`CellsFormatter`] that both filters and maps.
    pub fn filter_map<U>(&mut self, f: impl FnOnce(&T) -> Option<&U>) -> CellsFormatter<'_, 'b, U> {
        CellsFormatter {
            w: self.w,
            d: self.d.and_then(f),
            stretch: self.stretch,
        }
    }

    /// Creates a [`CellsFormatter`] that both filters and maps.
    pub fn filter_map_with<U>(
        &mut self,
        f: impl FnOnce(&'b T) -> Option<U>,
        some: impl FnOnce(&mut CellsFormatter<U>),
    ) {
        some(&mut CellsFormatter {
            w: self.w,
            d: self.d.and_then(f).as_ref(),
            stretch: self.stretch,
        });
    }

    pub fn try_map_with<O, E: RawCell>(
        &mut self,
        f: impl FnOnce(&'b T) -> std::result::Result<O, E>,
        ok: impl FnOnce(&mut CellsFormatter<O>),
    ) {
        let d = self.d.map(f);
        if let Some(Err(e)) = &d {
            self.w.content_start(e);
        }
        ok(&mut CellsFormatter {
            w: self.w,
            d: d.as_ref().and_then(|x| x.as_ref().ok()),
            stretch: self.stretch,
        });
        if let Some(Err(e)) = &d {
            self.w.content_end(e);
        }
    }

    /// Return `CellsFormatter` that generates the columns to be stretched preferentially.
    ///
    /// See [`ColumnStyle::stretch`](crate::ColumnStyle::stretch) for details.
    pub fn stretch(&mut self) -> CellsFormatter<'_, 'b, T> {
        CellsFormatter {
            w: self.w,
            d: self.d,
            stretch: true,
        }
    }

    /// Apply `f` to self.
    pub fn with(&mut self, f: impl Fn(&mut Self)) {
        f(self);
    }
}
impl<'a, 'b, T: ?Sized> CellsFormatter<'a, 'b, &T> {
    pub fn unref(&mut self) -> CellsFormatter<T> {
        CellsFormatter {
            w: self.w,
            d: self.d.map(|x| &**x),
            stretch: self.stretch,
        }
    }
}
impl<'a, 'b, T: ?Sized> CellsFormatter<'a, 'b, &mut T> {
    pub fn unref(&mut self) -> CellsFormatter<T> {
        CellsFormatter {
            w: self.w,
            d: self.d.map(|x| &**x),
            stretch: self.stretch,
        }
    }
}

pub(crate) trait CellsWrite {
    /// Called once for each cell.
    /// In the case of merged cells, it is also called for each unmerged cells.
    ///
    /// `cell`: Cell's value. If `None`, it is merged cells.
    fn content(&mut self, cell: Option<&dyn RawCell>, stretch: bool);

    /// Called when merged cells start.
    fn content_start(&mut self, cell: &dyn RawCell);

    /// Called when merged cells end.
    fn content_end(&mut self, cell: &dyn RawCell);

    /// Called at the start of cells separated by ruled lines.
    fn column_start(&mut self, header: &dyn RawCell);

    /// Called at the end of cells separated by ruled lines.
    fn column_end(&mut self, header: &dyn RawCell);
}
