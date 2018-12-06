#![cfg(test)]
use std::fmt::*;
use text_grid::TextGrid;

#[test]
fn cell_1() -> Result {
    let mut g = TextGrid::new();
    g.push_cell("aaa", Alignment::Right)?;
    g.finish_row();
    print!("{}", g);

    let e = " aaa |\n";
    let a = format!("{}", g);
    assert_eq!(a, e);

    Ok(())
}

#[test]
fn separator() -> Result {
    let mut g = TextGrid::new();
    g.push_cell("aaa", Alignment::Right)?;
    g.finish_row_with(true);
    print!("{}", g);

    let e = " aaa |\n-----|\n";
    let a = format!("{}", g);
    assert_eq!(a, e);

    Ok(())
}
