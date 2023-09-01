fn main() {
    use text_grid::*;
    struct RowData {
        a: u32,
        b_1: u32,
        b_2: u32,
    }
    impl CellsSource for RowData {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |s| s.a);
            f.column_with("b", |f| {
                f.content(|s| s.b_1);
                f.content(|_| " ");
                f.content(|s| s.b_2);
            });
        }
    }

    let mut g = Grid::new();
    g.push(&RowData {
        a: 300,
        b_1: 10,
        b_2: 20,
    });
    g.push(&RowData {
        a: 300,
        b_1: 1,
        b_2: 500,
    });

    print!("{}", g);
}
