use std::fmt::Alignment::*;
use std::fmt::*;
use text_grid::TextGrid;

fn main() -> Result {
    let mut g = TextGrid::new();
    g.row()
        .cell("name", Right)?
        .cell("value1", Right)?
        .cell("value2", Right)?
        .cell("value3", Right)?;

    g.row_separator();

    g.row()
        .cell("root", Right)?
        .cell(10, Right)?
        .cell(5, Right)?
        .cell("a", Left)?;

    g.row()
        .cell("p1", Right)?
        .cell(1, Right)?
        .cell(20, Right)?
        .cell("b", Left)?;

    print!("{}", g);

    Ok(())
}
