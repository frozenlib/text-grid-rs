fn main() {
    use text_grid::*;
    let mut g = GridBuilder::new();
    g.push(|b| {
        b.push(cell("name").right());
        b.push("type");
        b.push("value");
    });
    g.push_separator();
    g.push(|b| {
        b.push(cell(String::from("X")).right());
        b.push("A");
        b.push(10);
    });
    g.push(|b| {
        b.push(cell("Y").right());
        b.push_with_colspan(cell("BBB").center(), 2);
    });
    print!("{}", g);
}
