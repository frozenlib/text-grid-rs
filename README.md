# text-grid-rs

[![Crates.io](https://img.shields.io/crates/v/text-grid.svg)](https://crates.io/crates/text-grid)
[![Docs.rs](https://docs.rs/text-grid/badge.svg)](https://docs.rs/crate/text-grid)
[![Build Status](https://travis-ci.org/frozenlib/text-grid-rs.svg?branch=master)](https://travis-ci.org/frozenlib/text-grid-rs)

A library to create formatted plain-text tables.

## Example

```rust :main.rs
use text_grid::*;

fn main() {
    struct RowData {
        a: u32,
        b: u32,
    }
    impl RowSource for RowData {
        fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>) {
            w.column("a", |s| s.a);
            w.column("b", |s| s.b);
        }
    }

    let mut g = Grid::new();
    g.push_row(&RowData { a: 300, b: 1 });
    g.push_row(&RowData { a: 2, b: 200 });

    println!("{}", g);
}
```
Output:
```text
  a  |  b  |
-----|-----|
 300 |   1 |
   2 | 200 |
```

## License
This project is dual licensed under Apache-2.0/MIT. See the two LICENSE-* files for details.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
