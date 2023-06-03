use text_grid::*;

#[test]
fn column_u8() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
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

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
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

    impl<'s> CellsSource for Source<'s> {
        fn fmt(f: &mut CellsFormatter<&Self>) {
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

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
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

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.group("g", |f| {
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
    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.column("a", |s| s.a);
            f.group("b", |f| {
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
    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.column("a", |s| s.a);
            f.group("b", |f| {
                f.column("1", |s| s.b_1);
                f.column("2", |s| s.b_2);
            });
            f.group("c", |f| {
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

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.group("g", |f| {
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

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.column("a", |&x| cell_by(move |f| write!(f, "{:.2}", x.a)).right());
            f.column("b", |&x| x.b);
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

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.column("a", |&x| cell!("{:.2}", x.a).right());
            f.column("b", |&x| x.b);
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
fn map() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.map(|x| x.a).column("a", |&x| x);
            f.map(|x| x.b).column("b", |&x| x);
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
fn map_ref() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.map(|x| &x.a).column("a", |&x| x);
            f.map(|x| &x.b).column("b", |&x| x);
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
fn impl_debug() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.column("a", |x| x.a);
            f.column("b", |x| x.b);
        }
    }

    let mut g = Grid::new();
    g.push(&Source { a: 100, b: 200 });
    g.push(&Source { a: 1, b: 2 });

    let d = format!("{:?}", g);
    let e = r"
  a  |  b  |
-----|-----|
 100 | 200 |
   1 |   2 |
";
    assert_eq!(d.trim(), e.trim());
}

#[test]
fn with_schema() {
    struct MyGridSchema {
        len: usize,
    }

    impl GridSchema<[u32]> for MyGridSchema {
        fn fmt(&self, f: &mut CellsFormatter<&[u32]>) {
            for i in 0..self.len {
                f.column(i, |s| s[i]);
            }
        }
    }

    let mut g = Grid::new_with_schema(MyGridSchema { len: 3 });
    g.push(&[1, 2, 3]);
    g.push(&[4, 5, 6]);

    let d = format!("{:?}", g);
    let e = r"
 0 | 1 | 2 |
---|---|---|
 1 | 2 | 3 |
 4 | 5 | 6 |
";
    assert_eq!(d.trim(), e.trim());
}

#[test]
fn baseline() {
    struct Source {
        a: f64,
        b: String,
    }

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
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

    impl CellsSource for Source {
        fn fmt(f: &mut CellsFormatter<&Self>) {
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
    let schema = grid_schema::<Vec<u32>>(move |f| {
        for i in 0..max_colunm_count {
            f.column(i, |x| x.get(i));
        }
    });
    let mut g = Grid::new_with_schema(schema);
    g.extend(rows);
    assert_eq!(format!("\n{g}"), OUTPUT);
    const OUTPUT: &str = r"
 0 | 1 | 2 | 3 |
---|---|---|---|
 1 | 2 | 3 |   |
 1 | 2 |   |   |
 1 | 2 | 3 | 4 |
";
}

fn do_test<T: CellsSource>(s: Vec<T>, e: &str) {
    do_test_with_schema(s, DefaultGridSchema::default(), e);
}

fn do_test_with_schema<T>(s: Vec<T>, schema: impl GridSchema<T>, e: &str) {
    let mut g = Grid::new_with_schema(schema);
    for s in s {
        g.push(&s);
    }
    let a = format!("{}", g);
    let e = e.trim_matches('\n');
    let a = a.trim_matches('\n');
    assert!(a == e, "\nexpected :\n{}\nactual :\n{}\n", e, a);
}
