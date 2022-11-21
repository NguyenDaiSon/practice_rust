/// This program is to learn about loop
///
/// Unlike the other kinds of loops in Rust (`while`, `while let`, and `for`), `loop`s can be used as expressions
/// that return values via `break`.
///

fn main() {
    let mut i = 1;
    let i = loop {
        if i > 100 {
            break i;
        }
        i *= 2;
    }; // need to add ';'
    println!("i is {i}");
}
