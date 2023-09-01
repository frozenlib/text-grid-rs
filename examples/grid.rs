fn main() {
    use text_grid::*;
    struct RowData {
        a: u32,
        b: u32,
    }
    impl Cells for RowData {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |s| s.a);
            f.column("b", |s| s.b);
        }
    }

    let mut g = Grid::new();
    g.push(&RowData { a: 300, b: 1 });
    g.push(&RowData { a: 2, b: 200 });

    print!("{}", g);
}
