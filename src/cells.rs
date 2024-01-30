use std::marker::PhantomData;

use derive_ex::derive_ex;

use crate::{CellsFormatter, RawCell};

/// A data structure that can be formatted into cells.
///
/// The number of columns must be statically determined from the type.
///
/// If the number of columns is dynamically determined, [`CellsSchema`] must be used. See [`cells_schema`] for details.
pub trait Cells {
    /// Define columns. see [`CellsFormatter`] for details.
    fn fmt(f: &mut CellsFormatter<Self>);
}
impl Cells for () {
    fn fmt(_: &mut CellsFormatter<Self>) {}
}
impl<T: ?Sized + Cells> Cells for &T {
    fn fmt(f: &mut CellsFormatter<Self>) {
        T::fmt(&mut f.unref());
    }
}
impl<T: ?Sized + Cells> Cells for &mut T {
    fn fmt(f: &mut CellsFormatter<Self>) {
        T::fmt(&mut f.unref());
    }
}
impl<T: Cells, const N: usize> Cells for [T; N] {
    fn fmt(f: &mut CellsFormatter<Self>) {
        for i in 0..N {
            f.column(i, |x| &x[i]);
        }
    }
}
impl<T: Cells> Cells for Option<T> {
    fn fmt(f: &mut CellsFormatter<Self>) {
        f.filter_map(|x| x.as_ref()).content(|x| x)
    }
}
impl<T: Cells, E: RawCell> Cells for std::result::Result<T, E> {
    fn fmt(f: &mut CellsFormatter<Self>) {
        f.try_map_with(|x| x.as_ref(), |f| T::fmt(&mut f.unref()));
    }
}

/// Column definitions.
///
/// Define columns using [`CellsFormatter`].
///
/// To dynamically create a `CellsSchema`, use [`cells_schema`].
///
/// # Examples
/// ```
/// use text_grid::*;
///
/// struct MyCellsSchema {
///     len: usize,
/// }
///
/// impl CellsSchema for MyCellsSchema {
///     type Source = [u32; 3];
///     fn fmt(&self, f: &mut CellsFormatter<[u32; 3]>) {
///         for i in 0..self.len {
///             f.column(i, |s| s[i]);
///         }
///     }
/// }
///
/// let rows = [
///    [1, 2, 3],
///    [4, 5, 6],
/// ];
/// let schema = MyCellsSchema { len: 3 };
/// let g = to_grid_with_schema(rows, schema);
///
/// assert_eq!(format!("\n{g}"), r#"
///  0 | 1 | 2 |
/// ---|---|---|
///  1 | 2 | 3 |
///  4 | 5 | 6 |
/// "#);
/// ```
pub trait CellsSchema {
    type Source: ?Sized;

    /// Define column information. see [`CellsFormatter`] for details.
    fn fmt(&self, f: &mut CellsFormatter<Self::Source>);
}

/// Extension trait for [`CellsSchema`].
pub trait CellsSchemaExt: CellsSchema {
    fn as_ref(&self) -> impl CellsSchema<Source = &Self::Source> {
        self.map_ref()
    }
    fn map_ref<'a>(self) -> impl CellsSchema<Source = &'a Self::Source>
    where
        Self::Source: 'a;
}
impl<T> CellsSchemaExt for T
where
    T: CellsSchema,
{
    fn map_ref<'a>(self) -> impl CellsSchema<Source = &'a Self::Source>
    where
        Self::Source: 'a,
    {
        cells_schema(move |f| self.fmt(&mut f.map(|x| *x)))
    }
}

impl<T: CellsSchema> CellsSchema for Vec<T> {
    type Source = T::Source;
    fn fmt(&self, f: &mut CellsFormatter<Self::Source>) {
        for s in self {
            s.fmt(f);
        }
    }
}
impl<T: CellsSchema> CellsSchema for [T] {
    type Source = T::Source;
    fn fmt(&self, f: &mut CellsFormatter<Self::Source>) {
        for s in self {
            s.fmt(f);
        }
    }
}
impl<T: ?Sized + CellsSchema> CellsSchema for &T {
    type Source = T::Source;
    fn fmt(&self, f: &mut CellsFormatter<Self::Source>) {
        T::fmt(self, f)
    }
}

/// [`CellsSchema`] implementation that use [`Cells`].
#[derive(Clone, Copy, Debug)]
#[derive_ex(Default(bound()))]
pub struct DefaultCellsSchema<T: ?Sized>(PhantomData<T>);

impl<T: Cells + ?Sized> CellsSchema for DefaultCellsSchema<T> {
    type Source = T;
    fn fmt(&self, f: &mut CellsFormatter<Self::Source>) {
        T::fmt(f);
    }
}

/// Create [`CellsSchema`] from closure.
///
/// # Examples
///
/// By calculating the number of columns at runtime and creating a schema,
/// it is possible to create tables where the number of columns cannot be obtained statically.
///
/// ```rust
/// use text_grid::*;
/// let rows = [vec![1, 2, 3], vec![1, 2], vec![1, 2, 3, 4]];
/// let max_colunm_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);
/// let schema = cells_schema::<Vec<u32>>(move |f| {
///     for i in 0..max_colunm_count {
///         f.column(i, |x| x.get(i));
///     }
/// });
/// let g = to_grid_with_schema(rows, schema);
/// assert_eq!(format!("\n{g}"), OUTPUT);
///
/// const OUTPUT: &str = r"
///  0 | 1 | 2 | 3 |
/// ---|---|---|---|
///  1 | 2 | 3 |   |
///  1 | 2 |   |   |
///  1 | 2 | 3 | 4 |
/// ";
/// ```
pub fn cells_schema<T: ?Sized>(
    fmt: impl Fn(&mut CellsFormatter<T>),
) -> impl CellsSchema<Source = T> {
    struct FnCellsSchema<T: ?Sized, F> {
        fmt: F,
        _phantom: PhantomData<fn(&mut CellsFormatter<T>)>,
    }

    impl<T: ?Sized, F: Fn(&mut CellsFormatter<T>)> CellsSchema for FnCellsSchema<T, F> {
        type Source = T;
        fn fmt(&self, f: &mut CellsFormatter<T>) {
            (self.fmt)(f)
        }
    }
    FnCellsSchema {
        fmt,
        _phantom: PhantomData,
    }
}

macro_rules! impl_for_tuple {
    ($($idx:tt : $ty:ident,)*) => {
        impl<$($ty),*> Cells for ($($ty,)*) where $($ty: Cells),* {
            fn fmt(f: &mut CellsFormatter<Self>) {
                $(
                    f.map_with(|x| &x.$idx, Cells::fmt);
                )*
            }
        }

        impl<$($ty),*> CellsSchema for ($($ty,)*)
        where
            $($ty: CellsSchema, $ty::Source: Sized,)*
        {
            type Source = ($($ty::Source,)*);
            fn fmt(&self, f: &mut CellsFormatter<Self::Source>) {
                $(self.$idx.fmt(&mut f.map(|x| &x.$idx));)*
            }
        }
    };
}

impl_for_tuple!(0: T0,);
impl_for_tuple!(0: T0, 1: T1,);
impl_for_tuple!(0: T0, 1: T1, 2: T2,);
impl_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3,);
impl_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4,);
impl_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5,);
impl_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6,);
impl_for_tuple!(0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7,);
impl_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
);
impl_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
);
impl_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
);
impl_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
    11: T11,
);
impl_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
    11: T11,
    12: T12,
);
impl_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
    11: T11,
    12: T12,
    13: T13,
);
impl_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
    11: T11,
    12: T12,
    13: T13,
    14: T14,
);
impl_for_tuple!(
    0: T0,
    1: T1,
    2: T2,
    3: T3,
    4: T4,
    5: T5,
    6: T6,
    7: T7,
    8: T8,
    9: T9,
    10: T10,
    11: T11,
    12: T12,
    13: T13,
    14: T14,
    15: T15,
);
