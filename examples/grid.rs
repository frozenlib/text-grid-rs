fn main() {
    use text_grid::*;
    struct RowData {
        a: u32,
        b: u32,
    }
    impl ColumnSource for RowData {
        fn fmt(w: &mut ColumnFormatter<&Self>) {
            w.column("a", |s| s.a);
            w.column("b", |s| s.b);
        }
    }

    let mut g = Grid::new();
    g.push_row(&RowData { a: 300, b: 1 });
    g.push_row(&RowData { a: 2, b: 200 });

    print!("{}", g);
}
