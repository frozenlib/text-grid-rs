# text-grid-rs

[![Crates.io](https://img.shields.io/crates/v/text-grid.svg)](https://crates.io/crates/text-grid)
[![Docs.rs](https://docs.rs/text-grid/badge.svg)](https://docs.rs/text-grid)
[![CI](https://github.com/frozenlib/text-grid-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/frozenlib/text-grid-rs/actions/workflows/ci.yml)

A library to create formatted plain-text tables.

## Example

```rust :main.rs
use text_grid::*;
struct RowData {
    a: String,
    b: u32,
    c: u32,
    d: f64,
}
impl Cells for RowData {
    fn fmt(f: &mut CellsFormatter<Self>) {
        f.column("a", |s| &s.a);
        f.column("b", |s| s.b);
        f.column("c", |s| cell(s.c).left());
        f.column_with("d", |f| {
            f.column("x", |s| s.d);
            f.column("y", |s| cells_f!("{:.2e}", s.d));
        });
    }
}

let rows = [
    RowData { a: "ABC".to_string(), b: 300, c: 1,   d: 100.1 },
    RowData { a: "XY".to_string(),  b: 2,   c: 200, d: 1.234 },
];
let g = to_grid(rows);
assert_eq!(format!("\n{g}"), OUTPUT);

const OUTPUT: &str = r#"
  a  |  b  |  c  |         d          |
-----|-----|-----|--------------------|
     |     |     |    x    |    y     |
-----|-----|-----|---------|----------|
 ABC | 300 | 1   | 100.1   | 1.00 e 2 |
 XY  |   2 | 200 |   1.234 | 1.23 e 0 |
"#;
```

## License

This project is dual licensed under Apache-2.0/MIT. See the two LICENSE-\* files for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
