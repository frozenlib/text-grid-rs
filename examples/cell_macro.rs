fn main() {
    use text_grid::*;
    struct RowData {
        a: f64,
        b: f64,
    }
    impl RowSource for RowData {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.column("a", |s| cell!("{:.2}", s.a).right());
            w.column("b", |s| cell!("{:.3}", s.b).right());
        }
    }

    let mut g = Grid::new();
    g.push_row(&RowData { a: 1.10, b: 1.11 });
    g.push_row(&RowData { a: 1.00, b: 0.10 });

    print!("{}", g);
}
