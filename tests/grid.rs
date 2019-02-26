use text_grid::*;

#[test]
fn column_u8() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.column("a", |x| x.a);
            w.column("b", |x| x.b);
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

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.column("a", |x| x.a);
            w.column("b", |x| &x.b);
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

    impl<'s> RowSource for Source<'s> {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>)
        where
            's: 'a,
        {
            w.column("a", |x| x.s);
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

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.column("a", |x| x.a);
            w.column("p", |_| "xxx");
            w.column("b", |x| x.b);
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

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.group("g").with(|w| {
                w.column("a", |x| x.a);
                w.column("b", |x| x.b);
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
    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.column("a", |s| s.a);
            w.group("b").with(|w| {
                w.column("1", |s| s.b_1);
                w.column("2", |s| s.b_2);
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
    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.column("a", |s| s.a);
            w.group("b").with(|w| {
                w.column("1", |s| s.b_1);
                w.column("2", |s| s.b_2);
            });
            w.group("c").with(|w| {
                w.content(|s| s.c_1);
                w.content(|s| s.c_2);
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

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.group("g").with(|w| {
                w.content(|x| x.a);
                w.content(|x| x.b);
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
        b: f64,
    }
    use std::fmt::*;

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.column("a", |x| cell_by(move |w| write!(w, "{:.2}", x.a)).right());
            w.column("b", |x| x.b);
        }
    }

    do_test(
        vec![Source { a: 10.0, b: 10.1 }, Source { a: 1.22, b: 3.45 }],
        r"
   a   |  b   |
-------|------|
 10.00 | 10.1 |
  1.22 | 3.45 |
",
    );
}

#[test]
fn column_cell_macro() {
    struct Source {
        a: f64,
        b: f64,
    }

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.column("a", |x| cell!("{:.2}", x.a).right());
            w.column("b", |x| x.b);
        }
    }

    do_test(
        vec![Source { a: 10.0, b: 10.1 }, Source { a: 1.22, b: 3.45 }],
        r"
   a   |  b   |
-------|------|
 10.00 | 10.1 |
  1.22 | 3.45 |
",
    );
}

#[test]
fn map() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.map(|x| x.a).column("a", |x| x);
            w.map(|x| x.b).column("b", |x| x);
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

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.map(|x| &x.a).column("a", |x| x);
            w.map(|x| &x.b).column("b", |x| x);
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

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<Source = &'a Self>) {
            w.column("a", |x| x.a);
            w.column("b", |x| x.b);
        }
    }

    let mut g = Grid::new();
    g.push_row(&Source { a: 100, b: 200 });
    g.push_row(&Source { a: 1, b: 2 });

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
        fn fmt_row<'a>(&self, w: &mut impl RowWrite<Source = &'a [u32]>) {
            for i in 0..self.len {
                w.column(i, |s| s[i]);
            }
        }
    }

    let mut g = Grid::new_with_schema(MyGridSchema { len: 3 });
    g.push_row(&[1, 2, 3]);
    g.push_row(&[4, 5, 6]);

    let d = format!("{:?}", g);
    let e = r"
 0 | 1 | 2 |
---|---|---|
 1 | 2 | 3 |
 4 | 5 | 6 |
";
    assert_eq!(d.trim(), e.trim());
}

fn do_test<T: RowSource>(s: Vec<T>, e: &str) {
    let mut g = Grid::new();
    for s in s {
        g.push_row(&s);
    }
    let a = format!("{}", g);
    let e = e.trim_matches('\n');
    let a = a.trim_matches('\n');
    assert!(a == e, "\nexpected :\n{}\nactual :\n{}\n", e, a);
}
