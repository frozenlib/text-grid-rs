use text_grid::*;

#[test]
fn column_u8() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |x| x.a);
            f.column("b", |x| x.b);
        }
    }

    do_test(
        vec![Source { a: 100, b: 200 }, Source { a: 1, b: 2 }],
        r"
  a  |  b  |
-----|-----|
 100 | 200 |
   1 |   2 |
",
    );
}

#[test]
fn column_u8_ref() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |x| x.a);
            f.column("b", |x| &x.b);
        }
    }

    do_test(
        vec![Source { a: 100, b: 200 }, Source { a: 1, b: 2 }],
        r"
  a  |  b  |
-----|-----|
 100 | 200 |
   1 |   2 |
",
    );
}

#[test]
fn column_str() {
    struct Source<'s> {
        s: &'s str,
    }

    impl Cells for Source<'_> {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |x| x.s);
        }
    }

    do_test(
        vec![Source { s: "aaa" }, Source { s: "bbb" }],
        r"
  a  |
-----|
 aaa |
 bbb |
",
    );
}

#[test]
fn column_static_str() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |x| x.a);
            f.column("p", |_| "xxx");
            f.column("b", |x| x.b);
        }
    }

    do_test(
        vec![Source { a: 100, b: 200 }, Source { a: 1, b: 2 }],
        r"
  a  |  p  |  b  |
-----|-----|-----|
 100 | xxx | 200 |
   1 | xxx |   2 |
",
    );
}

#[test]
fn column_group() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column_with("g", |f| {
                f.column("a", |x| x.a);
                f.column("b", |x| x.b);
            })
        }
    }

    do_test(
        vec![Source { a: 100, b: 200 }, Source { a: 1, b: 2 }],
        r"
     g     |
-----------|
  a  |  b  |
-----|-----|
 100 | 200 |
   1 |   2 |
",
    );
}

#[test]
fn column_group_differing_level() {
    struct Source {
        a: u32,
        b_1: u32,
        b_2: u32,
    }
    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |s| s.a);
            f.column_with("b", |f| {
                f.column("1", |s| s.b_1);
                f.column("2", |s| s.b_2);
            });
        }
    }

    do_test(
        vec![
            Source {
                a: 300,
                b_1: 10,
                b_2: 20,
            },
            Source {
                a: 300,
                b_1: 1,
                b_2: 500,
            },
        ],
        r"
  a  |    b     |
-----|----------|
     | 1  |  2  |
-----|----|-----|
 300 | 10 |  20 |
 300 |  1 | 500 |",
    );
}

#[test]
fn column_group_differing_level_2() {
    struct Source {
        a: u32,
        b_1: u32,
        b_2: u32,
        c_1: u32,
        c_2: u32,
    }
    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |s| s.a);
            f.column_with("b", |f| {
                f.column("1", |s| s.b_1);
                f.column("2", |s| s.b_2);
            });
            f.column_with("c", |f| {
                f.content(|s| s.c_1);
                f.content(|s| s.c_2);
            });
        }
    }

    do_test(
        vec![
            Source {
                a: 300,
                b_1: 10,
                b_2: 20,
                c_1: 5,
                c_2: 6,
            },
            Source {
                a: 300,
                b_1: 1,
                b_2: 500,
                c_1: 7,
                c_2: 8,
            },
        ],
        r"
  a  |    b     | c  |
-----|----------|----|
     | 1  |  2  |    |
-----|----|-----|----|
 300 | 10 |  20 | 56 |
 300 |  1 | 500 | 78 |",
    );
}

#[test]
fn column_multipart() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column_with("g", |f| {
                f.content(|x| x.a);
                f.content(|x| x.b);
            })
        }
    }

    do_test(
        vec![Source { a: 10, b: 200 }, Source { a: 1, b: 2 }],
        r"
   g   |
-------|
 10200 |
  1  2 |
",
    );
}

#[test]
fn column_cell_by() {
    struct Source {
        a: f64,
        b: u32,
    }
    use std::fmt::*;

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |x| cell_by(move |f| write!(f, "{:.2}", x.a)).right());
            f.column("b", |x| x.b);
        }
    }

    do_test(
        vec![Source { a: 10.0, b: 30 }, Source { a: 1.22, b: 40 }],
        r"
   a   | b  |
-------|----|
 10.00 | 30 |
  1.22 | 40 |
",
    );
}

#[test]
fn column_cell_macro() {
    struct Source {
        a: f64,
        b: u32,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |x| cell!("{:.2}", x.a).right());
            f.column("b", |x| x.b);
        }
    }

    do_test(
        vec![Source { a: 10.0, b: 30 }, Source { a: 1.22, b: 40 }],
        r"
   a   | b  |
-------|----|
 10.00 | 30 |
  1.22 | 40 |
",
    );
}

#[test]
fn column_with_schema() {
    let s0 = cells_schema::<i32>(|f| f.content(|x| *x + 1));
    struct X {
        a: i32,
        b: i32,
    }
    let s1 = cells_schema::<X>(|f| {
        f.column_with_schema("a", |x| x.a, &s0);
        f.column_with_schema("b", |x| &x.b, s0.as_ref());
    });
    let x = vec![X { a: 1, b: 2 }, X { a: 4, b: 5 }];
    do_test_with_schema(
        x,
        s1,
        r"
 a | b |
---|---|
 2 | 3 |
 5 | 6 |
",
    );
}

#[test]
fn map_with_value() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.map_with(|x| x.a, |f| f.column("a", |x| x));
            f.map_with(|x| x.b, |f| f.column("b", |x| x));
        }
    }

    do_test(
        vec![Source { a: 100, b: 200 }, Source { a: 1, b: 2 }],
        r"
  a  |  b  |
-----|-----|
 100 | 200 |
   1 |   2 |
",
    );
}

#[test]
fn map_with_ref() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.map_with(|x| &x.a, |f| f.column("a", |x| x));
            f.map_with(|x| &x.b, |f| f.column("b", |x| x));
        }
    }

    do_test(
        vec![Source { a: 100, b: 200 }, Source { a: 1, b: 2 }],
        r"
  a  |  b  |
-----|-----|
 100 | 200 |
   1 |   2 |
",
    );
}

#[test]
fn map() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.map(|x| &x.a).column("a", |x| x);
            f.map(|x| &x.b).column("b", |x| x);
        }
    }

    do_test(
        vec![Source { a: 100, b: 200 }, Source { a: 1, b: 2 }],
        r"
  a  |  b  |
-----|-----|
 100 | 200 |
   1 |   2 |
",
    );
}

#[test]
fn with_schema() {
    struct MyCellsSchema {
        len: usize,
    }

    impl CellsSchema for MyCellsSchema {
        type Source = [u32; 3];
        fn fmt(&self, f: &mut CellsFormatter<[u32; 3]>) {
            for i in 0..self.len {
                f.column(i, |s| s[i]);
            }
        }
    }

    let rows = [[1, 2, 3], [4, 5, 6]];
    let g = to_grid_with_schema(rows, MyCellsSchema { len: 3 });
    let e = r"
 0 | 1 | 2 |
---|---|---|
 1 | 2 | 3 |
 4 | 5 | 6 |
";
    assert_eq!(g.trim(), e.trim());
}

#[test]
fn right() {
    let s = cells_schema::<&str>(|f| f.column("x", |x| cell(x).right()));
    do_test_with_schema(
        vec!["a", "ab", "abc"],
        &s,
        r"
  x  |
-----|
   a |
  ab |
 abc |",
    );
}

#[test]
fn baseline() {
    struct Source {
        a: f64,
        b: String,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |x| x.a);
            f.column("b", |x| cell(&x.b).baseline("-"));
        }
    }

    do_test(
        vec![
            Source {
                a: 100.1,
                b: "1-2345".into(),
            },
            Source {
                a: 10.123,
                b: "1234-5".into(),
            },
        ],
        r"
    a    |     b     |
---------|-----------|
 100.1   |    1-2345 |
  10.123 | 1234-5    |",
    );
}

#[test]
fn root_content() {
    struct Source {
        a: u32,
        b: u32,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.content(|x| x.a);
            f.content(|_| " ");
            f.content(|x| x.b);
        }
    }

    do_test(
        vec![Source { a: 10, b: 1 }, Source { a: 30, b: 100 }],
        r"
 10   1 |
 30 100 |",
    );
}

#[test]
fn disparate_column_count() {
    let rows = vec![vec![1, 2, 3], vec![1, 2], vec![1, 2, 3, 4]];
    let max_colunm_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let schema = cells_schema::<Vec<u32>>(move |f| {
        for i in 0..max_colunm_count {
            f.column(i, |x| x.get(i));
        }
    });

    let g = to_grid_with_schema(rows, schema);
    assert_eq!(format!("\n{g}"), OUTPUT);

    const OUTPUT: &str = r"
 0 | 1 | 2 | 3 |
---|---|---|---|
 1 | 2 | 3 |   |
 1 | 2 |   |   |
 1 | 2 | 3 | 4 |
";
}

#[test]
fn cell_ref() {
    let _ = cells_schema(|f: &mut CellsFormatter<&String>| {
        f.column("x", |x| cell!("__{}__", x));
    });
}

#[test]
fn cells_f() {
    let s = cells_schema::<f64>(|f| {
        f.column("", |x| cell!("{x:e}"));
        f.column("e", |x| cells_f!("{x:e}"));
        f.column(".2e", |x| cells_f!("{x:.2e}"));
        f.column("E", |x| cells_f!("{x:E}"));
        f.column("debug", |x| cells_f!("{x:?}"));
    });

    do_test_with_schema(
        vec![1.0, 0.95, 123.45, 0.000001, 1.0e-20, 10000000000.0],
        s,
        r"
          |      e       |    .2e     |      E       |        debug         |
----------|--------------|------------|--------------|----------------------|
 1e0      | 1      e   0 | 1.00 e   0 | 1      E   0 |           1.0        |
 9.5e-1   | 9.5    e  -1 | 9.50 e  -1 | 9.5    E  -1 |           0.95       |
 1.2345e2 | 1.2345 e   2 | 1.23 e   2 | 1.2345 E   2 |         123.45       |
 1e-6     | 1      e  -6 | 1.00 e  -6 | 1      E  -6 |           1    e  -6 |
 1e-20    | 1      e -20 | 1.00 e -20 | 1      E -20 |           1    e -20 |
 1e10     | 1      e  10 | 1.00 e  10 | 1      E  10 | 10000000000.0        |
",
    );
}

#[test]
fn cells_f_padding() {
    let s = cells_schema::<f64>(|f| {
        f.column("+++++++++", |x| cells_f!("{x}"));
    });

    do_test_with_schema(
        vec![1.0],
        s,
        r"
 +++++++++ |
-----------|
         1 |
",
    );
}

#[test]
fn empty_group() {
    let s = cells_schema::<()>(|f| {
        f.column_with("header", |_| {});
        f.column("1", |_| cell(1));
    });

    do_test_with_schema(
        vec![()],
        s,
        r"
 1 |
---|
 1 |
",
    );
}

#[test]
fn result() {
    do_test(
        vec![Ok(["a", "b"]), Err("********")],
        r"
  0  | 1  |
-----|----|
 a   | b  |
 ******** |
",
    );
}

#[test]
fn zero_rows() {
    struct Source {
        a: u8,
        b: u8,
    }
    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |x| x.a);
            f.column("b", |x| x.b);
        }
    }

    do_test(
        Vec::<Source>::new(),
        r"
 a | b |
---|---|
",
    );
}

#[test]
fn zero_rows_column_group() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column_with("g", |f| {
                f.column("a", |x| x.a);
                f.column("b", |x| x.b);
            })
        }
    }

    do_test(
        Vec::<Source>::new(),
        r"
   g   |
-------|
 a | b |
---|---|
",
    );
}

#[test]
fn zero_rows_colspan() {
    struct Source {
        a: f64,
    }

    impl Cells for Source {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("x", |x| x.a);
        }
    }

    do_test(
        Vec::<Source>::new(),
        r"
 x |
---|
",
    );
}

#[test]
fn index_to_value_schema() {
    struct IndexToValueSchema<T>(Vec<T>);

    impl<T: Cells> CellsSchema for IndexToValueSchema<T> {
        type Source = usize;

        fn fmt(&self, f: &mut CellsFormatter<Self::Source>) {
            f.column("header", |&index| &self.0[index])
        }
    }
    do_test_with_schema(
        vec![0, 1, 2],
        IndexToValueSchema(vec!["a", "b", "c"]),
        r"
 header |
--------|
 a      |
 b      |
 c      |",
    );
}

#[test]
fn to_grid_with_schema_array() {
    struct X(u8);

    impl Cells for X {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("x", |x| x.0);
        }
    }

    let schema = cells_schema(|f: &mut CellsFormatter<X>| {
        f.column("x", |x| x.0);
    });

    to_grid_with_schema([X(1), X(2)], schema);
}

#[test]
fn to_grid_with_schema_slice() {
    struct X(u8);

    impl Cells for X {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("x", |x| x.0);
        }
    }

    let schema = cells_schema(|f: &mut CellsFormatter<X>| {
        f.column("x", |x| x.0);
    });

    to_grid_with_schema(&[X(1), X(2)], schema);
}

#[test]
fn derive_cells_for_record_struct() {
    #[derive(Cells)]
    struct X {
        a: String,
        b: u32,
    }

    do_test(
        vec![
            X {
                a: "aaa".into(),
                b: 100,
            },
            X {
                a: "bbb".into(),
                b: 200,
            },
        ],
        r"
  a  |  b  |
-----|-----|
 aaa | 100 |
 bbb | 200 |
",
    );
}

#[test]
fn derive_cells_for_tuple_struct() {
    #[derive(Cells)]
    struct X(String, u32);

    do_test(
        vec![X("aaa".into(), 100), X("bbb".into(), 200)],
        r"
 aaa100 |
 bbb200 |
",
    );
}

#[test]
fn derive_cells_for_enum() {
    #[derive(Cells)]
    enum X {
        A,
        B,
        C,
    }

    do_test(
        vec![X::A, X::B, X::C],
        r"
 A |
 B |
 C |
",
    );
}

#[test]
fn derive_cells_for_enum_tuple_field() {
    #[derive(Cells)]
    enum X {
        A,
        B(u32),
        C(u32, u32),
    }

    do_test(
        vec![X::A, X::B(1), X::C(2, 3)],
        r"
   | 0 | 1 |
---|---|---|
 A |   |   |
 B | 1 |   |
 C | 2 | 3 |
",
    );
}

#[test]
fn derive_cells_for_enum_record_field() {
    #[derive(Cells)]
    enum X {
        A,
        B { x: u32 },
        C { x: u32, y: u32 },
    }

    do_test(
        vec![X::A, X::B { x: 1 }, X::C { x: 2, y: 3 }],
        r"
   | x | y |
---|---|---|
 A |   |   |
 B | 1 |   |
 C | 2 | 3 |
",
    );
}

#[test]
fn derive_cells_for_enum_both_field2() {
    #[derive(Cells)]
    enum X {
        A,
        B(u32),
        C { x: u32, y: u32 },
    }

    let g = to_grid(vec![X::A, X::B(1), X::C { x: 2, y: 3 }]);
    assert_eq!(
        format!("\n{g}"),
        r"
   | 0 | x | y |
---|---|---|---|
 A |   |   |   |
 B | 1 |   |   |
 C |   | 2 | 3 |
",
    );
}

#[test]
fn derive_cells_for_enum_both_field() {
    #[derive(Cells)]
    enum X {
        A,
        B(u32),
        C { x: u32, y: u32 },
    }

    do_test(
        vec![X::A, X::B(1), X::C { x: 2, y: 3 }],
        r"
   | 0 | x | y |
---|---|---|---|
 A |   |   |   |
 B | 1 |   |   |
 C |   | 2 | 3 |
",
    );
}

#[test]
fn derive_cells_generic() {
    #[derive(Cells)]
    struct X<T>(T);

    do_test(
        vec![X(1), X(2), X(3)],
        r"
 1 |
 2 |
 3 |
",
    );
}

#[track_caller]
fn do_test<T: Cells>(s: Vec<T>, e: &str) {
    do_test_with_schema(s, DefaultCellsSchema::default(), e);
}

#[track_caller]
fn do_test_with_schema<T>(s: Vec<T>, schema: impl CellsSchema<Source = T>, e: &str) {
    let a = to_grid_with_schema(s, schema).to_string();
    let e = e.trim_matches('\n');
    let a = a.trim_matches('\n');
    assert!(a == e, "\nexpected :\n{}\nactual :\n{}\n", e, a);
}
