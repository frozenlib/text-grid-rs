#![cfg(test)]
use std::fmt::Alignment::*;
use std::fmt::*;
use text_grid::TextGrid;

#[test]
fn cell_1() -> Result {
    let mut g = TextGrid::new();
    g.push_row().push("aaa", Right)?;
    let e = r"
 aaa |";
    do_test(g, e)
}

#[test]
fn colspan_2() -> Result {
    let mut g = TextGrid::new();
    g.push_row()
        .push("xxx", Center)?
        .push_merged(1)
        .push("end", Center)?;
    g.push_row()
        .push("1", Left)?
        .push("2", Left)?
        .push("3", Left)?;

    let e = r"
  xxx  | end |
 1 | 2 | 3   |";
    do_test(g, e)
}

#[test]
fn colspan_3() -> Result {
    let mut g = TextGrid::new();
    g.push_row()
        .push("title", Center)?
        .push_merged(2)
        .push("end", Center)?;
    g.push_row()
        .push("1", Left)?
        .push("2", Left)?
        .push("3", Left)?
        .push("4", Left)?;

    let e = r"
   title   | end |
 1 | 2 | 3 | 4   |";
    do_test(g, e)
}

#[test]
fn separator() -> Result {
    let mut g = TextGrid::new();
    g.push_row().push("aaa", Right)?;
    g.push_separator();
    g.push_row().push("aaa", Right)?;

    let e = r"
 aaa |
-----|
 aaa |";
    do_test(g, e)
}

fn do_test(g: TextGrid, e: &str) -> Result {
    let a = format!("{}", g);
    let e = e.trim_matches('\n');
    let a = a.trim_matches('\n');
    assert_eq!(a, e);
    Ok(())
}
