fn main() {
    use text_grid::*;
    struct RowData {
        a: f64,
        b: f64,
    }
    impl CellsSource for RowData {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.column("a", |&s| cell!("{:.2}", s.a).right());
            f.column("b", |&s| cell!("{:.3}", s.b).right());
        }
    }

    let mut g = Grid::new();
    g.push(&RowData { a: 1.10, b: 1.11 });
    g.push(&RowData { a: 1.00, b: 0.10 });

    print!("{}", g);

    panic!("failed");
}
