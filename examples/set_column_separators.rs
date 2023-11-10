use pretty_assertions::assert_eq;

fn main() {
    use text_grid::*;
    let mut g = GridBuilder::new();
    g.push(|b| {
        b.push("A");
        b.push("B");
        b.push("C");
    });
    assert_eq!(format!("\n{g}"), E0);

    g.column_styles = vec![ColumnStyle::default(); 2];
    g.column_styles[0].column_end = false;

    assert_eq!(format!("\n{g}"), E1);

    const E0: &str = r"
 A | B | C |
";

    const E1: &str = r"
 AB | C |
";
}
