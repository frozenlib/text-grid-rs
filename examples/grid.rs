use std::fmt::*;
use text_grid::TextGrid;

fn main() -> Result {
    let mut g = TextGrid::new();
    g.push_cell("name", Alignment::Right)?;
    g.push_cell("value1", Alignment::Right)?;
    g.push_cell("value2", Alignment::Right)?;
    g.push_cell("value3", Alignment::Right)?;
    g.finish_row_with(true);

    g.push_cell("root", Alignment::Right)?;
    g.push_cell(10, Alignment::Right)?;
    g.push_cell(5, Alignment::Right)?;
    g.push_cell("a", Alignment::Left)?;
    g.finish_row();

    g.push_cell("p1", Alignment::Right)?;
    g.push_cell(1, Alignment::Right)?;
    g.push_cell(20, Alignment::Right)?;
    g.push_cell("b", Alignment::Left)?;
    g.finish_row();

    print!("{}", g);

    Ok(())
}
