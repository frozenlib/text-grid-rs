fn main() {
    use text_grid::*;
    let mut g = GridBuilder::new();
    g.push(|b| {
        b.push("A");
        b.push("B");
        b.push("C");
    });
    g.push(|b| {
        b.push("AAA");
        b.push("BBB");
        b.push("CCC");
    });
    g.set_column_separators(vec![true, true]);
    println!("{:?}", vec![true, true]);
    println!("{}", g);

    g.set_column_separators(vec![false, true]);
    println!("{:?}", vec![false, true]);
    println!("{}", g);
}
