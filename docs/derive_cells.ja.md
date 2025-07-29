`Cells` トレイトの自動実装を提供するDerive マクロ

# 基本機能

## レコード構造体

レコード構造体では、各フィールドがテーブルの列として表示されます。フィールド名がカラムヘッダーとなり、フィールドの値がセルの内容となります。

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

出力結果：

```
 name  | age |
-------|-----|
 Alice |  25 |
 Bob   |  30 |
```

## タプル構造体

タプル構造体では、通常すべてのフィールドが連結されて単一のセルの内容として表示されます。列の分離は行われません。ただし、フィールドに`#[cells(header = "...")]`を指定した場合は、そのフィールドが個別の列として分離されます。

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

出力結果：

```
 Apple 100 |
 Banana 80 |
```

## 列挙型

列挙型では、最初の列にバリアント名が表示され、各バリアントのフィールドがその後に配置されます。異なるバリアント間で同じ名前または同じ位置のフィールドは、同じ列に整列されます。

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

出力結果：

```
       | 0     | x  | y  |
-------|-------|----|----|
 Text  | hello |    |    |
 Click |       | 10 | 20 |
 Move  |       |  5 | 15 |
```

### 制限事項

同じ名前または同じ位置のフィールドは、すべて同じ型である必要があります。異なる型のフィールドが同じ位置にある場合、コンパイルエラーが発生します。

```rust
// コンパイルエラーとなる例
#[derive(Cells)]
enum Invalid {
    Text(String),   // 位置0にString型
    Number(u32),    // 位置0にu32型 - エラー！
}
```

# フィールド属性

フィールドレベルで使用できる属性により、テーブル表示の動作を詳細にカスタマイズできます。

- [`#[cells(header = "...")]`](#cellsheader--) - カラムヘッダーのカスタマイズ
- [`#[cells(body = ...)]`](#cellsbody--) - セル表示内容のカスタマイズ
- [`#[cells(skip)]`](#cellsskip) - フィールドの表示除外

## `#[cells(header = "...")]`

デフォルトのフィールド名の代わりに、カスタムのカラムヘッダーを指定できます。指定する式は[`RawCell`]を実装する必要があります。

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

出力結果：

```
 Full Name | Years |
-----------|-------|
 Alice     |    25 |
 Bob       |    30 |
```

## `#[cells(body = ...)]`

セルに表示される内容をカスタマイズできます。式内では`self`を使用してフィールド値にアクセスできます。指定する式は[`Cells`]を実装する必要があります。

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

出力結果：

```
    name     |     age      |
-------------|--------------|
 Name: Alice | 25 years old |
 Name: Bob   | 30 years old |
```

## `#[cells(skip)]`

指定したフィールドをテーブル表示から完全に除外します。機密情報や内部状態など、表示が不適切なフィールドに使用します。

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

出力結果：

```
 name  | age |
-------|-----|
 Alice |  25 |
 Bob   |  30 |
```