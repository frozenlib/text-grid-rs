fn main() {
    use text_grid::*;
    struct RowData {
        a: String,
        b: u32,
        c: u32,
        d: f64,
    }
    impl Cells for RowData {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |s| &s.a);
            f.column("b", |s| s.b);
            f.column("c", |s| cell(s.c).left());
            f.column_with("d", |f| {
                f.column("x", |s| s.d);
                f.column("y", |s| cells_f!("{:.2e}", s.d));
            });
        }
    }

    let mut g = Grid::new();
    g.push(&RowData {
        a: "ABC".to_string(),
        b: 300,
        c: 1,
        d: 100.1,
    });
    g.push(&RowData {
        a: "XY".to_string(),
        b: 2,
        c: 200,
        d: 1.234,
    });
    println!("\n{g}");
    assert_eq!(format!("\n{g}"), OUTPUT);
}

const OUTPUT: &str = r#"
  a  |  b  |  c  |         d          |
-----|-----|-----|--------------------|
     |     |     |    x    |    y     |
-----|-----|-----|---------|----------|
 ABC | 300 | 1   | 100.1   | 1.00 e 2 |
 XY  |   2 | 200 |   1.234 | 1.23 e 0 |
"#;
