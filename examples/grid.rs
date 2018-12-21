fn main() {
    use text_grid::*;
    struct RowData {
        a: u32,
        b: u32,
    }
    impl RowSource for RowData {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source=&'a Self>) {
            w.column("a", |s| s.a);
            w.column("b", |s| s.b);
        }
    }

    let mut g = Grid::new();
    g.push_row(&RowData { a: 300, b: 1 });
    g.push_row(&RowData { a: 2, b: 200 });

    print!("{}", g);
}
