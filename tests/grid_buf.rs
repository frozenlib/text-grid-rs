use text_grid::*;

#[test]
fn cell_1() {
    let mut g = GridBuilder::new();
    g.push_row(|b| b.push("aaa"));
    let e = r"
 aaa |";
    do_test(g, e);
}

#[test]
fn cell_u8() {
    let mut g = GridBuilder::new();
    g.push_row(|b| b.push(10u8));
}

#[test]
fn cell_option_u8() {
    let mut g = GridBuilder::new();
    g.push_row(|b| {
        b.push(Some(10u8));
    });
}

#[test]
fn cell_option_ref_u8() {
    let mut g = GridBuilder::new();
    g.push_row(|b| {
        b.push(10u8);
    });
}
#[test]
fn colspan_2() {
    let mut g = GridBuilder::new();
    g.push_row(|b| {
        b.push_with_colspan(cell("xxx").center(), 2);
        b.push(cell("end").center());
    });
    g.push_row(|b| {
        b.push("1");
        b.push("2");
        b.push("3");
    });

    let e = r"
  xxx  | end |
 1 | 2 | 3   |";
    do_test(g, e);
}

#[test]
fn colspan_3() {
    let mut g = GridBuilder::new();
    g.push_row(|b| {
        b.push_with_colspan(cell("title").center(), 3);
        b.push(cell("end"));
    });
    g.push_row(|b| {
        b.push("1");
        b.push("2");
        b.push("3");
        b.push("4");
    });

    let e = r"
   title   | end |
 1 | 2 | 3 | 4   |";
    do_test(g, e);
}

#[test]
fn separator() {
    let mut g = GridBuilder::new();
    g.push_row(|b| {
        b.push(cell("aaa").right());
    });
    g.push_separator();
    g.push_row(|b| {
        b.push(cell("aaa").right());
    });

    let e = r"
 aaa |
-----|
 aaa |";
    do_test(g, e);
}
fn do_test(g: GridBuilder, e: &str) {
    let a = format!("{}", g);
    let e = e.trim_matches('\n');
    let a = a.trim_matches('\n');
    assert!(a == e, "\nexpected :\n{}\nactual :\n{}\n", e, a);
}
