use text_grid::{to_grid_with_schema, Cells, CellsSchema, DefaultCellsSchema};

#[track_caller]
fn check<T: Cells>(s: Vec<T>, e: &str) {
    check_with_schema(s, DefaultCellsSchema::default(), e);
}

#[track_caller]
fn check_with_schema<T>(s: Vec<T>, schema: impl CellsSchema<Source = T>, e: &str) {
    let a = to_grid_with_schema(s, schema).to_string();
    let e = e.trim_matches('\n');
    let a = a.trim_matches('\n');
    assert!(a == e, "\nexpected :\n{e}\nactual :\n{a}\n");
}

#[test]
fn derive_record_struct() {
    #[derive(Cells)]
    struct Person {
        name: &'static str,
        age: u32,
        active: bool,
    }

    check(
        vec![
            Person {
                name: "Alice",
                age: 25,
                active: true,
            },
            Person {
                name: "Bob",
                age: 30,
                active: false,
            },
        ],
        r"
 name  | age | active |
-------|-----|--------|
 Alice |  25 |  true  |
 Bob   |  30 | false  |
",
    );
}

#[test]
fn derive_tuple_struct() {
    #[derive(Cells)]
    struct Point(f64, f64);

    check(
        vec![Point(1.5, 2.0), Point(3.0, 4.5)],
        r"
 1.52   |
 3  4.5 |
",
    );
}

#[test]
fn derive_unit_struct() {
    #[derive(Cells)]
    struct Empty;

    check(
        vec![Empty, Empty],
        r"

",
    );
}

#[test]
fn derive_single_field_struct() {
    #[derive(Cells)]
    struct Wrapper(String);

    check(
        vec![Wrapper("hello".to_string()), Wrapper("world".to_string())],
        r"
 hello |
 world |
",
    );
}

#[test]
fn derive_simple_enum() {
    #[derive(Cells)]
    enum Status {
        Active,
        Inactive,
        Pending,
    }

    check(
        vec![Status::Active, Status::Inactive, Status::Pending],
        r"
 Active   |
 Inactive |
 Pending  |
",
    );
}

#[test]
fn derive_enum_with_tuple_fields() {
    #[derive(Cells)]
    enum Message {
        Text(String),
        Number(String),
        Point(String, String),
    }

    check(
        vec![
            Message::Text("hello".to_string()),
            Message::Number("42".to_string()),
            Message::Point("1.0".to_string(), "2.0".to_string()),
        ],
        r"
        |   0   |  1  |
--------|-------|-----|
 Text   | hello |     |
 Number | 42    |     |
 Point  | 1.0   | 2.0 |
",
    );
}

#[test]
fn derive_enum_with_record_fields() {
    #[derive(Cells)]
    enum Event {
        Click { x: u32, y: u32 },
        KeyPress { key: char },
        Resize { width: u32, height: u32 },
    }

    check(
        vec![
            Event::Click { x: 10, y: 20 },
            Event::KeyPress { key: 'a' },
            Event::Resize {
                width: 800,
                height: 600,
            },
        ],
        r"
          | x  | y  | key | width | height |
----------|----|----|-----|-------|--------|
 Click    | 10 | 20 |     |       |        |
 KeyPress |    |    |  a  |       |        |
 Resize   |    |    |     |   800 |    600 |
",
    );
}

#[test]
fn derive_enum_mixed_fields() {
    #[derive(Cells)]
    enum Data {
        Empty,
        Single(String),
        Pair(String, String),
        Named { value: String, label: String },
    }

    check(
        vec![
            Data::Empty,
            Data::Single("10".to_string()),
            Data::Pair("1".to_string(), "2".to_string()),
            Data::Named {
                value: "42".to_string(),
                label: "answer".to_string(),
            },
        ],
        r"
        | 0  | 1 | value | label  |
--------|----|---|-------|--------|
 Empty  |    |   |       |        |
 Single | 10 |   |       |        |
 Pair   | 1  | 2 |       |        |
 Named  |    |   | 42    | answer |
",
    );
}

#[test]
fn derive_generic_struct() {
    #[derive(Cells)]
    struct Container<T> {
        value: T,
        name: String,
    }

    check(
        vec![
            Container {
                value: "hello",
                name: "first".to_string(),
            },
            Container {
                value: "world",
                name: "second".to_string(),
            },
        ],
        r"
 value |  name  |
-------|--------|
 hello | first  |
 world | second |
",
    );
}

#[test]
fn derive_generic_tuple_struct() {
    #[derive(Cells)]
    struct Pair<T, U>(T, U);

    check(
        vec![Pair("a", "1"), Pair("b", "2")],
        r"
 a1 |
 b2 |
",
    );
}

#[test]
fn derive_nested_struct() {
    #[derive(Cells)]
    struct Inner {
        value: u32,
    }

    #[derive(Cells)]
    struct Outer {
        inner: Inner,
        name: String,
    }

    check(
        vec![
            Outer {
                inner: Inner { value: 10 },
                name: "first".to_string(),
            },
            Outer {
                inner: Inner { value: 20 },
                name: "second".to_string(),
            },
        ],
        r"
 inner |  name  |
-------|--------|
 value |        |
-------|--------|
    10 | first  |
    20 | second |
",
    );
}

#[test]
fn derive_nested_enum() {
    #[derive(Cells)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Cells)]
    enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        Point(Point),
    }

    check(
        vec![
            Shape::Circle { radius: 5.0 },
            Shape::Rectangle {
                width: 10.0,
                height: 20.0,
            },
            Shape::Point(Point { x: 1, y: 2 }),
        ],
        r"
           |   0   | radius | width | height |
-----------|-------|--------|-------|--------|
           | x | y |        |       |        |
-----------|---|---|--------|-------|--------|
 Circle    |   |   |   5    |       |        |
 Rectangle |   |   |        |  10   |  20    |
 Point     | 1 | 2 |        |       |        |
",
    );
}

#[test]
fn derive_large_tuple_struct() {
    #[derive(Cells)]
    struct BigTuple(String, String, String, String, String);

    check(
        vec![
            BigTuple(
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ),
            BigTuple(
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
            ),
        ],
        r"
 abcde |
 12345 |
",
    );
}

#[test]
fn derive_many_fields_struct() {
    #[derive(Cells)]
    struct ManyFields {
        a: String,
        b: String,
        c: String,
        d: String,
        e: String,
    }

    check(
        vec![
            ManyFields {
                a: "a1".to_string(),
                b: "b1".to_string(),
                c: "c1".to_string(),
                d: "d1".to_string(),
                e: "e1".to_string(),
            },
            ManyFields {
                a: "a2".to_string(),
                b: "b2".to_string(),
                c: "c2".to_string(),
                d: "d2".to_string(),
                e: "e2".to_string(),
            },
        ],
        r"
 a  | b  | c  | d  | e  |
----|----|----|----|----|
 a1 | b1 | c1 | d1 | e1 |
 a2 | b2 | c2 | d2 | e2 |
",
    );
}

#[test]
fn derive_single_variant_enum() {
    #[derive(Cells)]
    enum Single {
        Only(String),
    }

    check(
        vec![Single::Only("test".to_string())],
        r"
      |  0   |
------|------|
 Only | test |
",
    );
}

#[test]
fn derive_custom_header() {
    #[derive(Cells)]
    struct Person {
        #[cells(header = "Full Name")]
        name: String,
        #[cells(header = "Years")]
        age: u32,
    }

    check(
        vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
        ],
        r"
 Full Name | Years |
-----------|-------|
 Alice     |    25 |
 Bob       |    30 |
",
    );
}

#[test]
fn derive_custom_body() {
    #[derive(Cells)]
    struct Person {
        #[cells(body = format!("Name: {}", self.name))]
        name: String,
        #[cells(body = format!("{} years old", self.age))]
        age: u32,
    }

    check(
        vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
        ],
        r"
    name     |     age      |
-------------|--------------|
 Name: Alice | 25 years old |
 Name: Bob   | 30 years old |
",
    );
}
