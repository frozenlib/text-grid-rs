fn main() {
    use text_grid::*;
    let mut g = GridBuilder::new();
    {
        let mut row = g.push_row();
        row.push(cell("name").right());
        row.push("type");
        row.push("value");
    }
    g.push_separator();
    {
        let mut row = g.push_row();
        row.push(cell(String::from("X")).right());
        row.push("A");
        row.push(10);
    }
    {
        let mut row = g.push_row();
        row.push(cell("Y").right());
        row.push_with_colspan(cell("BBB").center(), 2);
    }

    print!("{}", g);
}
