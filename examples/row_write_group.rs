fn main() {
    use text_grid::*;
    struct RowData {
        a: u32,
        b_1: u32,
        b_2: u32,
    }
    impl CellsSource for RowData {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.column("a", |s| s.a);
            f.group("b", |f| {
                f.column("1", |s| s.b_1);
                f.column("2", |s| s.b_2);
            });
        }
    }

    let mut g = Grid::new();
    g.push_row(&RowData {
        a: 300,
        b_1: 10,
        b_2: 20,
    });
    g.push_row(&RowData {
        a: 300,
        b_1: 1,
        b_2: 500,
    });

    print!("{}", g);
}
