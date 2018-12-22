use crate::cell::*;

pub trait RowWriteCore {
    fn group_start(&mut self);
    fn group_end(&mut self, header: impl CellSource);
}

/// Used to define column information within [`RowSource::fmt_row`](crate::RowSource::fmt_row).
///
/// - Use [`column`](Self::column) to create column.
/// - Use [`group`](Self::group) to create multi level header.
/// - Use [`content`](Self::content) to create shared header columns.
pub trait RowWrite: RowWriteCore {
    type Source;

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
    ///     fn fmt_row<'a>(w: &mut impl RowWrite<Source=&'a Self>) {
    ///         w.column("a", |s| s.a);
    ///         w.group("b").with(|w| {
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
    fn group<C: CellSource>(&mut self, header: C) -> GroupGuard<Self, C> {
        self.group_start();
        GroupGuard {
            w: self,
            header: Some(header),
        }
    }

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
    ///     fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
    ///         w.column("a", |s| s.a);
    ///         w.group("b").with(|w| {
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
    ///
    /// ```
    ///
    /// Output:
    /// ```text
    ///   a  |   b    |
    /// -----|--------|
    ///  300 | 10  20 |
    ///  300 |  1 500 |
    /// ```
    ///
    /// [`RowSource`]: #crate::RowSource
    fn content<T: CellSource>(&mut self, f: impl FnOnce(Self::Source) -> T);

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
    ///     fn fmt_row<'a>(w: &mut impl RowWrite<Source=&'a Self>) {
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
    ///
    /// [`RowSource`]: #crate::RowSource
    fn column<T: CellSource>(
        &mut self,
        header: impl CellSource,
        f: impl FnOnce(Self::Source) -> T,
    ) {
        self.group(header).content(f);
    }

    fn map<F: Fn(Self::Source) -> R, R>(&mut self, f: F) -> Map<Self, F> {
        Map { w: self, f }
    }
    fn filter<F: Fn(&Self::Source) -> bool>(&mut self, f: F) -> Filter<Self, F> {
        Filter { w: self, f }
    }
    fn filter_map<F: Fn(Self::Source) -> Option<R>, R>(&mut self, f: F) -> FilterMap<Self, F> {
        FilterMap { w: self, f }
    }
    fn with(&mut self, f: impl Fn(&mut Self)) {
        f(self);
    }
}

pub struct Map<'a, W: ?Sized, F> {
    w: &'a mut W,
    f: F,
}

impl<'a, R, W: RowWrite, F: Fn(W::Source) -> R> RowWrite for Map<'a, W, F> {
    type Source = R;

    fn content<T: CellSource>(&mut self, f: impl FnOnce(Self::Source) -> T) {
        let f0 = &self.f;
        self.w.content(|s| f(f0(s)))
    }
}

impl<'a, W: RowWriteCore, F> RowWriteCore for Map<'a, W, F> {
    fn group_start(&mut self) {
        self.w.group_start();
    }
    fn group_end(&mut self, header: impl CellSource) {
        self.w.group_end(header);
    }
}

pub struct Filter<'a, W: ?Sized, F> {
    w: &'a mut W,
    f: F,
}
impl<'a, W: RowWrite + ?Sized, F: Fn(&W::Source) -> bool> RowWrite for Filter<'a, W, F> {
    type Source = W::Source;

    fn content<T: CellSource>(&mut self, f: impl FnOnce(Self::Source) -> T) {
        let f0 = &self.f;
        self.w.content(|s| if f0(&s) { Some(f(s)) } else { None })
    }
}

impl<'a, W: RowWriteCore + ?Sized, F> RowWriteCore for Filter<'a, W, F> {
    fn group_start(&mut self) {
        self.w.group_start();
    }
    fn group_end(&mut self, header: impl CellSource) {
        self.w.group_end(header);
    }
}

pub struct FilterMap<'a, W: ?Sized, F> {
    w: &'a mut W,
    f: F,
}
impl<'a, R, W: RowWrite + ?Sized, F: Fn(W::Source) -> Option<R>> RowWrite for FilterMap<'a, W, F> {
    type Source = R;

    fn content<T: CellSource>(&mut self, f: impl FnOnce(Self::Source) -> T) {
        let f0 = &self.f;
        self.w.content(|s| f0(s).map(f))
    }
}

impl<'a, W: RowWriteCore + ?Sized, F> RowWriteCore for FilterMap<'a, W, F> {
    fn group_start(&mut self) {
        self.w.group_start();
    }
    fn group_end(&mut self, header: impl CellSource) {
        self.w.group_end(header);
    }
}

pub struct GroupGuard<'a, W: RowWriteCore + ?Sized, C: CellSource> {
    w: &'a mut W,
    header: Option<C>,
}
impl<'a, W: RowWrite + ?Sized, C: CellSource> RowWrite for GroupGuard<'a, W, C> {
    type Source = W::Source;

    fn content<T: CellSource>(&mut self, f: impl FnOnce(Self::Source) -> T) {
        self.w.content(f)
    }
}

impl<'a, W: RowWriteCore + ?Sized, C: CellSource> RowWriteCore for GroupGuard<'a, W, C> {
    fn group_start(&mut self) {
        self.w.group_start();
    }
    fn group_end(&mut self, header: impl CellSource) {
        self.w.group_end(header);
    }
}
impl<W: RowWriteCore + ?Sized, C: CellSource> Drop for GroupGuard<'_, W, C> {
    fn drop(&mut self) {
        self.w.group_end(self.header.take().unwrap())
    }
}
