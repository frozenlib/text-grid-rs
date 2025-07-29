Derive macro that provides automatic implementation of the `Cells` trait

# Core Functionality

## Record Struct

In record structs, each field is displayed as a separate table column. Field names become column headers, and field values become cell contents.

```rust
use text_grid::{to_grid, Cells};

#[derive(Cells)]
struct Person {
    name: String,
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
];

let grid = to_grid(people);
println!("{}", grid);
```

Output:

```
 name  | age |
-------|-----|
 Alice |  25 |
 Bob   |  30 |
```

## Tuple Struct

In tuple structs, all fields are typically concatenated and displayed as the content of a single cell. No column separation occurs. However, if a field has `#[cells(header = "...")]` specified, that field will be separated into its own column.

```rust
use text_grid::{to_grid, Cells};

#[derive(Cells)]
struct Product(String, u32);

let products = vec![
    Product("Apple".to_string(), 100),
    Product("Banana".to_string(), 80),
];

let grid = to_grid(products);
println!("{}", grid);
```

Output:

```
 Apple 100 |
 Banana 80 |
```

## Enum

In enums, the first column displays the variant name, and variant fields are placed in subsequent columns. Fields with the same name or position across different variants are aligned in the same column.

```rust
use text_grid::{to_grid, Cells};

#[derive(Cells)]
enum Message {
    Text(String),
    Click { x: u32, y: u32 },
    Move { x: u32, y: u32 },
}

let messages = vec![
    Message::Text("hello".to_string()),
    Message::Click { x: 10, y: 20 },
    Message::Move { x: 5, y: 15 },
];

let grid = to_grid(messages);
println!("{}", grid);
```

Output:

```
       | 0     | x  | y  |
-------|-------|----|----|
 Text  | hello |    |    |
 Click |       | 10 | 20 |
 Move  |       |  5 | 15 |
```

### Constraints

Fields with the same name or position must all have the same type. If fields of different types occupy the same position, a compilation error will occur.

```rust
// Compilation error example
#[derive(Cells)]
enum Invalid {
    Text(String),   // Position 0 has String type
    Number(u32),    // Position 0 has u32 type - Error!
}
```

# Field Attributes

Field-level attributes provide fine-grained control over table display behavior, allowing detailed customization of how data is presented.

- [`#[cells(header = "...")]`](#cellsheader--) - Column header customization
- [`#[cells(body = ...)]`](#cellsbody--) - Cell content customization
- [`#[cells(skip)]`](#cellsskip) - Field display exclusion

## `#[cells(header = "...")]`

Specifies a custom column header instead of using the default field name. The expression must implement [`RawCell`].

```rust
use text_grid::{to_grid, Cells};

#[derive(Cells)]
struct Person {
    #[cells(header = "Full Name")]
    name: String,
    #[cells(header = "Years")]
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
];

let grid = to_grid(people);
println!("{}", grid);
```

Output:

```
 Full Name | Years |
-----------|-------|
 Alice     |    25 |
 Bob       |    30 |
```

## `#[cells(body = ...)]`

Customizes the content displayed in cells. The expression can access field values using `self`. The expression must implement [`Cells`].

```rust
use text_grid::{to_grid, Cells};

#[derive(Cells)]
struct Person {
    #[cells(body = format!("Name: {}", self.name))]
    name: String,
    #[cells(body = format!("{} years old", self.age))]
    age: u32,
}

let people = vec![
    Person { name: "Alice".to_string(), age: 25 },
    Person { name: "Bob".to_string(), age: 30 },
];

let grid = to_grid(people);
println!("{}", grid);
```

Output:

```
    name     |     age      |
-------------|--------------|
 Name: Alice | 25 years old |
 Name: Bob   | 30 years old |
```

## `#[cells(skip)]`

Completely excludes the specified field from table display. This is useful for sensitive information or internal state that should not be visible.

```rust
use text_grid::{to_grid, Cells};

#[derive(Cells)]
struct Person {
    name: String,
    #[cells(skip)]
    password: String,
    age: u32,
}

let people = vec![
    Person { 
        name: "Alice".to_string(), 
        password: "secret123".to_string(),
        age: 25 
    },
    Person { 
        name: "Bob".to_string(), 
        password: "password456".to_string(),
        age: 30 
    },
];

let grid = to_grid(people);
println!("{}", grid);
```

Output:

```
 name  | age |
-------|-----|
 Alice |  25 |
 Bob   |  30 |
```