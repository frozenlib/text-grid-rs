use crate::{Cells, CellsFormatter};

use self::HorizontalAlignment::*;
use std::{cmp::min, fmt::*};

/// Cell`s style.
#[derive(Clone, Copy, Default)]

pub struct CellStyle {
    pub(crate) align_h: Option<HorizontalAlignment>,
}
impl CellStyle {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the styles in which the empty style has been replaced with the specified style.
    ///
    /// Judgment as to whether the style is empty or not is done for each individual element.
    pub fn or(self, style: CellStyle) -> CellStyle {
        CellStyle {
            align_h: self.align_h.or(style.align_h),
        }
    }

    pub fn align_h(self, value: HorizontalAlignment) -> Self {
        CellStyle {
            align_h: Some(value),
        }
    }
}

/// Horizontal alignments for cell's content.
#[derive(Clone, Copy)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

/// A data structure that can be formatted into a cell.
///
/// Normally, [`cell()`] or [`cell!`](crate::cell!) is used to create a value that implements `RawCell`.
///
/// If you implement `RawCell` for a type, you should also implement [`Cells`] for convenience.
///
/// ```
/// use text_grid::*;
/// struct X(String);
///
/// impl RawCell for X {
///     fn fmt(&self, s: &mut String) {
///         s.push_str(&self.0);
///     }
///     fn style(&self) -> CellStyle {
///         CellStyle::new().align_h(HorizontalAlignment::Right)
///     }
/// }
/// impl Cells for X {
///     fn fmt(f: &mut CellsFormatter<Self>) {
///         f.content(Cell::new);
///     }
/// }
/// ```
pub trait RawCell {
    /// Output the cell text to given buffer.
    fn fmt(&self, s: &mut String);

    /// Return cell's style.
    fn style(&self) -> CellStyle {
        CellStyle::default()
    }

    fn style_for_body(&self) -> CellStyle {
        CellStyle::default()
    }
}
impl RawCell for () {
    fn fmt(&self, _: &mut String) {}
}
impl<T: ?Sized + RawCell> RawCell for &T {
    fn fmt(&self, s: &mut String) {
        T::fmt(*self, s)
    }
    fn style(&self) -> CellStyle {
        T::style(*self)
    }
    fn style_for_body(&self) -> CellStyle {
        T::style_for_body(*self)
    }
}
impl<T: ?Sized + RawCell> RawCell for &mut T {
    fn fmt(&self, s: &mut String) {
        T::fmt(*self, s)
    }
    fn style(&self) -> CellStyle {
        T::style(*self)
    }
    fn style_for_body(&self) -> CellStyle {
        T::style_for_body(*self)
    }
}
impl<T: RawCell> RawCell for Option<T> {
    fn fmt(&self, s: &mut String) {
        if let Some(value) = self {
            value.fmt(s);
        }
    }
    fn style(&self) -> CellStyle {
        if let Some(value) = self {
            value.style()
        } else {
            CellStyle::default()
        }
    }
    fn style_for_body(&self) -> CellStyle {
        if let Some(value) = self {
            value.style_for_body()
        } else {
            CellStyle::default()
        }
    }
}
impl<T: RawCell, E: RawCell> RawCell for std::result::Result<T, E> {
    fn fmt(&self, s: &mut String) {
        match self {
            Ok(value) => value.fmt(s),
            Err(value) => value.fmt(s),
        }
    }
    fn style(&self) -> CellStyle {
        match self {
            Ok(value) => value.style(),
            Err(value) => value.style(),
        }
    }
    fn style_for_body(&self) -> CellStyle {
        match self {
            Ok(value) => value.style_for_body(),
            Err(value) => value.style_for_body(),
        }
    }
}

struct DisplayCellSource<T: Display>(T);
impl<T: Display> RawCell for DisplayCellSource<T> {
    fn fmt(&self, s: &mut String) {
        write!(s, "{}", self.0).unwrap()
    }
}

/// Create [`Cell`] from [`Display`].
///
/// The returned value owns the value passed in.
/// Therefore, the returned value can not be move out of the lifetime of the passed value.
///
/// ```ignore
/// use text_grid::*;
///
/// fn f_error() -> Cell<impl RawCell> {
///     let s = String::from("ABC");
///     cell(&s) // Error : returns a value referencing data owned by the current function
/// }
/// ```
///
/// ```
/// use text_grid::*;
///
/// fn f_ok() -> Cell<impl RawCell> {
///     let s = String::from("ABC");
///     cell(s) // OK
/// }
/// ```
pub fn cell(value: impl Display) -> Cell<impl RawCell> {
    Cell::new(DisplayCellSource(value))
}

struct FmtFnCellSource<F>(F);
impl<F: Fn(&mut String) -> Result> RawCell for FmtFnCellSource<F> {
    fn fmt(&self, s: &mut String) {
        (self.0)(s).unwrap()
    }
}

/// Create [`Cell`] from closure that call [`std::write!`] macro.
///
/// # Examples
///
/// ```
/// use text_grid::*;
/// use std::fmt::Write;
///
/// let s = String::from("ABC");
/// let cell_a = cell_by(|f| write!(f, "{}", &s));
/// let cell_b = cell_by(|f| write!(f, "{}", &s));
/// ```
pub fn cell_by<F: Fn(&mut String) -> Result>(f: F) -> Cell<impl RawCell> {
    Cell::new(FmtFnCellSource(f))
}

/// Create [`Cell`] via runtime expression interpolation, as in [`format!`].
///
/// Use the `format!` syntax to create [`Cell`]. See [`std::fmt`] for more information.
///
/// # Examples
/// ```
/// use text_grid::*;
/// struct RowData {
///     a: f64,
///     b: f64,
/// }
/// impl Cells for RowData {
///     fn fmt(f: &mut CellsFormatter<Self>) {
///         f.column("a", |s| cell!("{:.2}", s.a).right());
///         f.column("b", |s| cell!("{:.3}", s.b).right());
///     }
/// }
///
/// let rows = [
///    RowData { a: 1.10, b: 1.11 },
///    RowData { a: 1.00, b: 0.10 },
/// ];
/// let g = to_grid(rows);
///
/// assert_eq!(format!("\n{g}"), r#"
///   a   |   b   |
/// ------|-------|
///  1.10 | 1.110 |
///  1.00 | 0.100 |
/// "#);
/// ```
///
/// # Arguments ownership
///
/// This macro consumes the variable used in the argument.
/// ```ignore
/// use text_grid::*;
///
/// let s = String::from("ABC");
/// let cell_a = cell!("{}", &s); // `s` moved into `cell_a` here
/// let cell_b = cell!("{}", &s); // ERROR : `s` used here after move
/// ```
///
/// To avoid consume variables, use only variables that implements `Copy` .
///
/// ```
/// use text_grid::*;
///
/// let s = String::from("ABC");
/// let s = &s; // immutable reference implements `Copy`.
/// let cell_a = cell!("{}", s);
/// let cell_b = cell!("{}", s); // OK
/// // Return value owns the reference.
/// // Therefore, the returned value can not be move out of the lifetime of the reference.
/// ```
///
/// or use [`cell_by`] and [`std::write!`].
///
/// ```
/// use text_grid::*;
/// use std::fmt::Write;
///
/// let s = String::from("ABC");
/// let cell_a = cell_by(|f| write!(f, "{}", &s));
/// let cell_b = cell_by(|f| write!(f, "{}", &s));
/// // Return value owns the reference.
/// // Therefore, the returned value can not be move out of the lifetime of the reference.
/// ```
///
/// or use [`cell()`] and [`std::format!`].
///
/// ```
/// use text_grid::*;
///
/// let s = String::from("ABC");
/// let cell_a = cell(format!("{}", &s));
/// let cell_b = cell(format!("{}", &s));
/// // Return value owns formatted string.
/// // Therefore, the returned value can move anywhere.
/// ```
#[macro_export]
macro_rules! cell {
    ($ ( $ arg : tt ) *) => { {
            use std::fmt::Write;
            $crate::cell_by(move |f| write!(f, $($arg)*))
        }
    };
}

/// Implementation of [`RawCell`] that can specify styles.
pub struct Cell<T> {
    source: T,
    style: CellStyle,
}
impl<T: RawCell> RawCell for Cell<T> {
    fn fmt(&self, s: &mut String) {
        self.source.fmt(s)
    }
    fn style(&self) -> CellStyle {
        self.style
    }
}
impl<T: RawCell> Cells for Cell<T> {
    fn fmt(f: &mut CellsFormatter<Self>) {
        f.content_cell(|s| s);
    }
}

impl<T: RawCell> Cell<T> {
    /// Create a new `Cell` with specified [`RawCell`].
    pub fn new(source: T) -> Self {
        let style = source.style();
        Cell { source, style }
    }

    /// Return the cell with horizontal alignment set to the left.
    pub fn left(self) -> Self {
        self.with_align_h(Left)
    }

    /// Return the cell with horizontal alignment set to the right.
    pub fn right(self) -> Self {
        self.with_align_h(Right)
    }

    /// Return the cell with horizontal alignment set to the center.
    pub fn center(self) -> Self {
        self.with_align_h(Center)
    }

    /// Return the cell with aligned baseline.
    ///
    /// ```rust
    /// use text_grid::*;
    ///
    /// struct Source(&'static str);
    ///
    /// impl Cells for Source {
    ///     fn fmt(f: &mut CellsFormatter<Self>) {
    ///         f.column("default", |x| &x.0);
    ///         f.column("baseline", |x| cell(&x.0).baseline("-"));
    ///     }
    /// }
    /// let rows = [
    ///     Source("1-2345"),
    ///     Source("1234-5"),
    ///     Source("12345"),
    /// ];
    /// let g = to_grid(rows);
    ///
    /// assert_eq!(format!("\n{g}"), r#"
    ///  default |  baseline  |
    /// ---------|------------|
    ///  1-2345  |     1-2345 |
    ///  1234-5  |  1234-5    |
    ///  12345   | 12345      |
    /// "#);
    /// ```
    pub fn baseline(self, baseline: &str) -> impl Cells {
        let mut value = String::new();
        self.fmt(&mut value);
        BaselineAlignedCell::new(value, baseline)
    }

    /// Return the cell with an empty style replaced by the specified style.
    ///
    /// Judgment as to whether the style is empty or not is done for each individual element.
    pub fn with_base_style(self, style: CellStyle) -> Self {
        Cell {
            source: self.source,
            style: self.style.or(style),
        }
    }
    fn with_align_h(self, align_h: HorizontalAlignment) -> Self {
        Cell {
            source: self.source,
            style: CellStyle {
                align_h: Some(align_h),
            },
        }
    }
}
impl Cell<String> {
    /// Create a new `Cell` with empty string and empty style.
    pub fn empty() -> Self {
        Self::new(String::new())
    }
}

macro_rules! impl_cell_source {
    ($t:ty, $align:expr ) => {
        impl RawCell for $t {
            fn fmt(&self, s: &mut String) {
                write!(s, "{}", self).unwrap()
            }
            fn style_for_body(&self) -> CellStyle {
                CellStyle {
                    align_h: Some($align),
                }
            }
        }
        impl Cells for $t {
            fn fmt(f: &mut CellsFormatter<Self>) {
                f.content_cell(|x| x);
            }
        }
    };
}

impl_cell_source!(u8, Right);
impl_cell_source!(i8, Right);
impl_cell_source!(u16, Right);
impl_cell_source!(i16, Right);
impl_cell_source!(u32, Right);
impl_cell_source!(i32, Right);
impl_cell_source!(u64, Right);
impl_cell_source!(i64, Right);
impl_cell_source!(u128, Right);
impl_cell_source!(i128, Right);
impl_cell_source!(isize, Right);
impl_cell_source!(usize, Right);
impl_cell_source!(String, Left);
impl_cell_source!(str, Left);
impl_cell_source!(char, Center);
impl_cell_source!(bool, Center);

/// A cell with aligned baseline.
///
/// Use [`Cell::baseline`] to create an instance of this type.
struct BaselineAlignedCell {
    value: String,
    baseline_offset: usize,
}
impl BaselineAlignedCell {
    fn new(value: String, baseline: &str) -> Self {
        let baseline_offset = value.find(baseline).unwrap_or(value.len());
        Self {
            value,
            baseline_offset,
        }
    }
    fn left(&self) -> &str {
        &self.value[..self.baseline_offset]
    }
    fn right(&self) -> &str {
        &self.value[self.baseline_offset..]
    }
}

impl Cells for BaselineAlignedCell {
    fn fmt(f: &mut CellsFormatter<Self>) {
        f.content(|this| cell(this.left()).right());
        f.content(|this| cell(this.right()).left());
    }
}

impl Cells for f32 {
    fn fmt(f: &mut CellsFormatter<Self>) {
        f.content(|this| cell(this).baseline("."))
    }
}

impl Cells for f64 {
    fn fmt(f: &mut CellsFormatter<Self>) {
        f.content(|this| cell(this).baseline("."))
    }
}

/// Create [`Cells`] for float numbers via runtime expression interpolation.
///
/// # Examples
///
/// ```
/// use text_grid::*;
/// let s = cells_schema::<f64>(|f| {
///     f.column("",      |x| cell!("{x:e}"));
///     f.column("e",     |x| cells_f!("{x:e}"));
///     f.column(".2e",   |x| cells_f!("{x:.2e}"));
///     f.column("E",     |x| cells_f!("{x:E}"));
///     f.column("debug", |x| cells_f!("{x:?}"));
/// });
///
/// let g = to_grid_with_schema(vec![1.0, 0.95, 123.45, 0.000001, 1.0e-20, 10000000000.0], s);
/// assert_eq!(format!("\n{g}"), OUTPUT);
///
/// const OUTPUT: &str = r"
///           |      e       |    .2e     |      E       |        debug         |
/// ----------|--------------|------------|--------------|----------------------|
///  1e0      | 1      e   0 | 1.00 e   0 | 1      E   0 |           1.0        |
///  9.5e-1   | 9.5    e  -1 | 9.50 e  -1 | 9.5    E  -1 |           0.95       |
///  1.2345e2 | 1.2345 e   2 | 1.23 e   2 | 1.2345 E   2 |         123.45       |
///  1e-6     | 1      e  -6 | 1.00 e  -6 | 1      E  -6 |           1    e  -6 |
///  1e-20    | 1      e -20 | 1.00 e -20 | 1      E -20 |           1    e -20 |
///  1e10     | 1      e  10 | 1.00 e  10 | 1      E  10 | 10000000000.0        |
/// ";
/// ```
#[macro_export]
macro_rules! cells_f {
    ($($t:tt)*) => {
        $crate::cells_f(format!($($t)*))
    };
}

/// Create [`Cells`] for float numbers from [`Display`].
///
/// Format in the same way as [`cells_f!`] macro.
pub fn cells_f(value: impl Display) -> impl Cells {
    ExpCells::new(value.to_string())
}
struct ExpCells {
    value: String,
    offset_dot: usize,
    offset_e: usize,
}
impl ExpCells {
    fn new(value: String) -> Self {
        let offset_e = value.rfind(['e', 'E']).unwrap_or(value.len());
        let offset_dot = min(value.find('.').unwrap_or(value.len()), offset_e);
        Self {
            value,
            offset_dot,
            offset_e,
        }
    }
}

impl Cells for ExpCells {
    fn fmt(f: &mut CellsFormatter<Self>) {
        f.stretch()
            .content(|x| cell(&x.value[..x.offset_dot]).right());
        f.content(|x| {
            if x.offset_dot < x.offset_e {
                &x.value[x.offset_dot..x.offset_e]
            } else {
                ""
            }
        });
        f.content(|x| {
            Cell::new(if x.offset_e < x.value.len() {
                Ok(cell!(" {} ", &x.value[x.offset_e..x.offset_e + 1]))
            } else {
                Err(())
            })
        });
        f.content(|x| cell(&x.value[min(x.offset_e + 1, x.value.len())..]).right());
    }
}
