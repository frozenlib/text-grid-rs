fn main() {
    use text_grid::*;
    struct RowData {
        a: u32,
        b_1: u32,
        b_2: u32,
    }
    impl RowSource for RowData {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.column("a", |s| s.a);
            w.group("b").with(|w| {
                w.column("1", |s| s.b_1);
                w.column("2", |s| s.b_2);
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
