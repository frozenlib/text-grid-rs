# text-grid-rs

[![Crates.io](https://img.shields.io/crates/v/text-grid.svg)](https://crates.io/crates/text-grid)
[![Docs.rs](https://docs.rs/text-grid/badge.svg)](https://docs.rs/text-grid)
[![CI](https://github.com/frozenlib/text-grid-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/frozenlib/text-grid-rs/actions/workflows/ci.yml)

A library to create formatted plain-text tables.

## Example

```rust :main.rs
use text_grid::*;

fn main() {
    struct RowData {
        a: u32,
        b: u32,
        c: f64
    }
    impl CellsSource for RowData {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.column("a", |s| s.a);
            f.column("b", |s| cell(s.b).left());
            f.column("c", |s| s.c);
        }
    }

    let mut g = Grid::new();
    g.push_row(&RowData { a: 300, b: 1, c: 100.1 });
    g.push_row(&RowData { a: 2, b: 200, c: 1.234 });

    assert_eq!(format!("\n{g}"), OUTPUT);
}

const OUTPUT: &str = r#"
  a  |  b  |    c    |
-----|-----|---------|
 300 | 1   | 100.1   |
   2 | 200 |   1.234 |
"#;
```

## License

This project is dual licensed under Apache-2.0/MIT. See the two LICENSE-\* files for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
