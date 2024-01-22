use text_grid::{Cells, CellsFormatter, Grid};

#[test]
fn to_csv() {
    struct X {
        a: u8,
        b: u8,
    }

    impl Cells for X {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |x| x.a);
            f.column("b", |x| x.b);
        }
    }
    let g = Grid::from_iter([X { a: 1, b: 2 }, X { a: 3, b: 4 }]);

    assert_eq!(g.to_csv(), "a,b\n1,2\n3,4\n");
}

#[test]
fn to_csv_nested() {
    struct X {
        a: u8,
        y: Y,
    }
    impl Cells for X {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("a", |x| x.a);
            f.column("y", |x| &x.y);
        }
    }

    struct Y {
        b: u8,
        c: u8,
    }
    impl Cells for Y {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.column("b", |x| x.b);
            f.column("c", |x| x.c);
        }
    }

    let g = Grid::from_iter([
        X {
            a: 1,
            y: Y { b: 2, c: 3 },
        },
        X {
            a: 4,
            y: Y { b: 5, c: 6 },
        },
    ]);

    assert_eq!(g.to_csv(), "a,y.b,y.c\n1,2,3\n4,5,6\n");
}
