fn main() {
    use text_grid::*;

    struct MyGridSchema {
        len: usize,
    }

    impl GridSchema<[u32]> for MyGridSchema {
        fn fmt_row<'a>(&self, w: &mut impl RowWrite<Source = &'a [u32]>) {
            for i in 0..self.len {
                w.column(format!("{}", i), |s| s[i]);
            }
        }
    }

    let mut g = Grid::new_with_schema(MyGridSchema { len: 3 });;
    g.push_row(&[1, 2, 3]);
    g.push_row(&[4, 5, 6]);

    print!("{}", g);
}
