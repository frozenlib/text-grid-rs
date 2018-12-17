use text_grid::*;

fn main() {}

// fn f_error() {
//     let s = String::from("ABC");
//     let _cell_a = cell!("{}", &s); // `s` moved into `_cell_a` here
//     let _cell_b = cell!("{}", &s); // ERROR : `s` used here after move
// }
pub fn f_ok() {
    let s = String::from("ABC");
    let s = &s;
    let _cell_a = cell!("{}", s);
    let _cell_b = cell!("{}", s); // OK
}
pub fn f_write() {
    use std::fmt::Write;

    let s = String::from("ABC");
    let _cell_a = cell_by(|w| write!(w, "{}", &s));
    let _cell_b = cell_by(|w| write!(w, "{}", &s));
}

pub fn f_format() {
    let s = String::from("ABC");
    let _cell_a = cell(format!("{}", &s));
    let _cell_b = cell(format!("{}", &s));
}
