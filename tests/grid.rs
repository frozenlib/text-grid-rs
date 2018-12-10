use text_grid::*;

#[test]
fn column_u8() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>) {
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
        fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>) {
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
fn column_static_str() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>) {
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
        fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>) {
            w.group("g", |w| {
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
fn column_multipart() {
    struct Source {
        a: u8,
        b: u8,
    }

    impl RowSource for Source {
        fn fmt_row<'a>(w: &mut impl RowWrite<'a, Self>) {
            w.group("g", |w| {
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
