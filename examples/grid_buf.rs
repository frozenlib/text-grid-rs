use std::fmt::*;
use text_grid::*;

fn main() -> Result {
    let mut g = GridBuf::new();
    {
        let mut row = g.push_row();
        row.push(Cell::empty());
        row.push_with_colspan(cell("value group").center(), 3);
    }
    g.push_separator();

    {
        let mut row = g.push_row();
        row.push(cell("name").right());
        row.push(cell("value1").right());
        row.push(cell("value2").right());
        row.push(cell("value3").right());
    }

    g.push_separator();

    {
        let mut row = g.push_row();
        row.push(cell("root").right());
        row.push(10);
        row.push(5);
        row.push("a");
    }

    {
        let mut row = g.push_row();
        row.push(cell("p1"));
        row.push(1);
        row.push(20);
        row.push("b");
    }

    print!("{}", g);

    Ok(())
}
