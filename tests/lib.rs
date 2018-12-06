#![cfg(test)]
use std::fmt::Alignment::*;
use std::fmt::*;
use text_grid::TextGrid;

#[test]
fn cell_1() -> Result {
    let mut g = TextGrid::new();
    g.row().cell("aaa", Right)?;
    print!("{}", g);

    let e = " aaa |\n";
    let a = format!("{}", g);
    assert_eq!(a, e);

    Ok(())
}

#[test]
fn separator() -> Result {
    let mut g = TextGrid::new();
    g.row().cell("aaa", Right)?;
    g.row_separator();
    print!("{}", g);

    let e = " aaa |\n-----|\n";
    let a = format!("{}", g);
    assert_eq!(a, e);

    Ok(())
}
