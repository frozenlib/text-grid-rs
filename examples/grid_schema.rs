fn main() {
    use text_grid::*;

    struct MyCellsSchema {
        len: usize,
    }

    impl CellsSchema for MyCellsSchema {
        type Source = [u32; 3];
        fn fmt(&self, f: &mut CellsFormatter<[u32; 3]>) {
            for i in 0..self.len {
                f.column(i, |s| s[i]);
            }
        }
    }

    let rows = [[1, 2, 3], [4, 5, 6]];
    print!("{}", to_grid(rows));
}
