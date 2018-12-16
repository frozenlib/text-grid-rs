fn main() {
    use text_grid::*;
    let mut g = GridBuf::new();
    {
        let mut row = g.push_row();
        row.push("A");
        row.push("B");
        row.push("C");
    }
    {
        let mut row = g.push_row();
        row.push("AAA");
        row.push("BBB");
        row.push("CCC");
    }
    g.set_column_separators(vec![true, true]);
    println!("{:?}", vec![true, true]);
    println!("{}", g);

    g.set_column_separators(vec![false, true]);
    println!("{:?}", vec![false, true]);
    println!("{}", g);
}
