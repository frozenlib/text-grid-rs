use text_grid::*;

fn main() {}

pub fn f_ok() -> Cell<impl RawCell> {
    let s = String::from("ABC");
    cell(s) // OK
}

// fn f_error() -> Cell<impl RawCell> {
//     let s = String::from("ABC");
//     cell(&s) // Error : returns a value referencing data owned by the current function
// }
