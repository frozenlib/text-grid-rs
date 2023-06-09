fn main() {
    use text_grid::*;

    struct MyGridSchema {
        len: usize,
    }

    impl GridSchema for MyGridSchema {
        type Source = [u32];
        fn fmt(&self, f: &mut CellsFormatter<&[u32]>) {
            for i in 0..self.len {
                f.column(i, |s| s[i]);
            }
        }
    }

    let mut g = Grid::new_with_schema(MyGridSchema { len: 3 });
    g.push(&[1, 2, 3]);
    g.push(&[4, 5, 6]);

    print!("{}", g);
}
