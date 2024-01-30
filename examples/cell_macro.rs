fn main() {
    use text_grid::*;
    struct RowData {
        a: f64,
        b: f64,
    }
    impl Cells for RowData {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |s| cell!("{:.2}", s.a).right());
            f.column("b", |s| cell!("{:.3}", s.b).right());
        }
    }

    let rows = vec![RowData { a: 300.0, b: 1.0 }, RowData { a: 2.0, b: 200.0 }];
    let g = to_grid(rows);

    print!("{}", g);

    panic!("failed");
}
