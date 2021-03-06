use self::HorizontalAlignment::*;
use std::fmt::*;

/// Cell`s style.
#[derive(Clone, Copy, Default)]
pub struct CellStyle {
    pub(crate) align_h: Option<HorizontalAlignment>,
}
impl CellStyle {
    /// Returns the styles in which the empty style has been replaced with the specified style.
    ///
    /// Judgment as to whether the style is empty or not is done for each individual element.
    pub fn or(self, style: CellStyle) -> CellStyle {
        CellStyle {
            align_h: self.align_h.or(style.align_h),
        }
    }
}

/// Possible horizontal alignments for cell's content.
#[derive(Clone, Copy)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

/// A data structure that can be formatted into cell.
///
/// Normally, [`cell()`] or [`macro@cell`] is used to create a value that implements `CellSource`.
pub trait CellSource {
    /// Output the cell text to given buffer.
    fn fmt(&self, s: &mut String);

    /// Return cell's style.
    fn style(&self) -> CellStyle {
        CellStyle::default()
    }

    /// Return cell's default style that associated with `Self` type.
    fn default_style() -> CellStyle {
        CellStyle::default()
    }
}
impl<T: CellSource> CellSource for &T {
    fn fmt(&self, s: &mut String) {
        T::fmt(*self, s)
    }
    fn style(&self) -> CellStyle {
        T::style(*self)
    }
    fn default_style() -> CellStyle {
        T::default_style()
    }
}
impl<T: CellSource> CellSource for &mut T {
    fn fmt(&self, s: &mut String) {
        T::fmt(*self, s)
    }
    fn style(&self) -> CellStyle {
        T::style(*self)
    }
    fn default_style() -> CellStyle {
        T::default_style()
    }
}
impl<T: CellSource> CellSource for Option<T> {
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
    fn default_style() -> CellStyle {
        T::default_style()
    }
}

struct DisplayCellSource<T: Display>(T);
impl<T: Display> CellSource for DisplayCellSource<T> {
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
/// fn f_error() -> Cell<impl CellSource> {
///     let s = String::from("ABC");
///     cell(&s) // Error : returns a value referencing data owned by the current function
/// }
/// ```
///
/// ```
/// use text_grid::*;
///
/// fn f_ok() -> Cell<impl CellSource> {
///     let s = String::from("ABC");
///     cell(s) // OK
/// }
/// ```
pub fn cell(value: impl Display) -> Cell<impl CellSource> {
    Cell::new(DisplayCellSource(value))
}

struct FmtFnCellSource<F>(F);
impl<F: Fn(&mut String) -> Result> CellSource for FmtFnCellSource<F> {
    fn fmt(&self, s: &mut String) {
        (self.0)(s).unwrap()
    }
}

/// Create [`Cell`] from closure that call `std::write!` macro.
///
/// # Examples
///
/// ```
/// use text_grid::*;
/// use std::fmt::Write;
///
/// let s = String::from("ABC");
/// let cell_a = cell_by(|w| write!(w, "{}", &s));
/// let cell_b = cell_by(|w| write!(w, "{}", &s));
/// ```
pub fn cell_by<F: Fn(&mut String) -> Result>(f: F) -> Cell<impl CellSource> {
    Cell::new(FmtFnCellSource(f))
}

/// Creates a [`Cell`] using interpolation of runtime expressions.
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
/// impl RowSource for RowData {
///     fn fmt_row<'a>(w: &mut impl RowWrite<Source=&'a Self>) {
///         w.column("a", |s| cell!("{:.2}", s.a).right());
///         w.column("b", |s| cell!("{:.3}", s.b).right());
///     }
/// }
///
/// let mut g = Grid::new();
/// g.push_row(&RowData { a: 1.10, b: 1.11 });
/// g.push_row(&RowData { a: 1.00, b: 0.10 });
///
/// print!("{}", g);
/// ```
///
/// Output:
/// ```text
///   a   |   b   |
/// ------|-------|
///  1.10 | 1.110 |
///  1.00 | 0.100 |
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
/// or use [`cell_by`] and `write!`.
///
/// ```
/// use text_grid::*;
/// use std::fmt::Write;
///
/// let s = String::from("ABC");
/// let cell_a = cell_by(|w| write!(w, "{}", &s));
/// let cell_b = cell_by(|w| write!(w, "{}", &s));
/// // Return value owns the reference.
/// // Therefore, the returned value can not be move out of the lifetime of the reference.
/// ```
///
/// or use [`cell()`] and `format!`.
///
/// ```
/// use text_grid::*;
///
/// let s = String::from("ABC");
/// let cell_a = cell(format!("{}", &s));
/// let cell_b = cell(format!("{}", &s));
/// // Retrun value owns formatted string.
/// // Therefore, the returned value can move anywhere.
/// ```
#[macro_export]
macro_rules! cell {
    ($ ( $ arg : tt ) *) => { {
            use std::fmt::Write;
            $crate::cell_by(move |w| write!(w, $($arg)*))
        }
    };
}

/// Implementation of [`CellSource`] that can specify styles.
pub struct Cell<T> {
    source: T,
    style: CellStyle,
}
impl<T: CellSource> CellSource for Cell<T> {
    fn fmt(&self, s: &mut String) {
        self.source.fmt(s)
    }
    fn style(&self) -> CellStyle {
        self.style
    }
}

impl<T: CellSource> Cell<T> {
    /// Create a new `Cell` with specified [`CellSource`] and empty style.
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
        impl CellSource for $t {
            fn fmt(&self, s: &mut String) {
                write!(s, "{}", self).unwrap()
            }
            fn default_style() -> CellStyle {
                CellStyle {
                    align_h: Some($align),
                }
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
impl_cell_source!(f32, Right);
impl_cell_source!(f64, Right);
impl_cell_source!(String, Left);
impl_cell_source!(&str, Left);
impl_cell_source!(char, Center);
impl_cell_source!(bool, Center);
