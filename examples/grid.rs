use std::fmt::Alignment::*;
use std::fmt::*;
use text_grid::TextGrid;

fn main() -> Result {
    let mut g = TextGrid::new();
    g.push_row()
        .push_empty()
        .push("value group", Center)?
        .push_merged(2);
    g.push_separator();

    g.push_row()
        .push("name", Right)?
        .push("value1", Right)?
        .push("value2", Right)?
        .push("value3", Right)?;

    g.push_separator();

    g.push_row()
        .push("root", Right)?
        .push(10, Right)?
        .push(5, Right)?
        .push("a", Left)?;

    g.push_row()
        .push("p1", Right)?
        .push(1, Right)?
        .push(20, Right)?
        .push("b", Left)?;

    print!("{}", g);

    Ok(())
}
