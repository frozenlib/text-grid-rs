fn main() {
    use text_grid::*;

    struct MyGridSchema {
        len: usize,
    }

    impl GridSchema<[u32]> for MyGridSchema {
        fn fmt(&self, f: &mut CellsFormatter<&[u32]>) {
            for i in 0..self.len {
                f.column(i, |s| s[i]);
            }
        }
    }

    let mut g = Grid::new_with_schema(MyGridSchema { len: 3 });
    g.push_row(&[1, 2, 3]);
    g.push_row(&[4, 5, 6]);

    print!("{}", g);
}
