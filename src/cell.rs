use self::HorizontalAlignment::*;
use std::fmt::*;

#[derive(Clone, Copy, Default)]
pub struct CellStyle {
    pub(crate) h_align: Option<HorizontalAlignment>,
}
impl CellStyle {
    pub fn or(self, style: CellStyle) -> CellStyle {
        CellStyle {
            h_align: self.h_align.or(style.h_align),
        }
    }
}

#[derive(Clone, Copy)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

pub trait CellSource {
    fn fmt(&self, s: &mut String);
    fn style(&self) -> CellStyle {
        CellStyle::default()
    }
    fn default_style() -> CellStyle {
        CellStyle::default()
    }

    fn to_owned(&self) -> Cell<String> {
        let mut source = String::new();
        self.fmt(&mut source);
        Cell {
            source,
            style: self.style(),
        }
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
            value.fmt(s)
        } else {
            ()
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
pub fn cell(value: impl Display) -> Cell<impl CellSource> {
    Cell::new(DisplayCellSource(value))
}

struct FmtFnCellSource<F>(F);
impl<F: Fn(&String) -> Result> CellSource for FmtFnCellSource<F> {
    fn fmt(&self, s: &mut String) {
        (self.0)(s).unwrap()
    }
}
pub fn cell_by_fmt<F: Fn(&String) -> Result>(f: F) -> Cell<impl CellSource> {
    Cell::new(FmtFnCellSource(f))
}

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
    pub fn new(source: T) -> Self {
        let style = source.style();
        Cell { source, style }
    }
    pub fn left(self) -> Self {
        self.with_h_align(Left)
    }
    pub fn right(self) -> Self {
        self.with_h_align(Right)
    }
    pub fn center(self) -> Self {
        self.with_h_align(Center)
    }
    pub fn with_base_style(self, style: CellStyle) -> Self {
        Cell {
            source: self.source,
            style: self.style.or(style),
        }
    }
    fn with_h_align(self, h_align: HorizontalAlignment) -> Self {
        Cell {
            source: self.source,
            style: CellStyle {
                h_align: Some(h_align),
                ..self.style
            },
        }
    }
}
impl Cell<String> {
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
                    h_align: Some($align),
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
