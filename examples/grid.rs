use std::fmt::*;
use text_grid::*;

fn main() -> Result {
    struct MySource {
        a: u8,
        b: u8,
    }
    impl RowSource for MySource {
        fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>) {
            w.column("a", |s| s.a);
            w.column("b", |s| &s.b);
        }
    }

    let mut g = Grid::new();
    g.push_row(&MySource { a: 100, b: 200 });
    g.push_row(&MySource { a: 1, b: 2 });

    print!("{}", g);

    Ok(())
}
