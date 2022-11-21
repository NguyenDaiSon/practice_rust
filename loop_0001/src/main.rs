/// This program is to learn about loop
fn main() {
    let mut i = 1;
    loop {
        println!("i is {i}");
        if i > 100 {
            break;
        }
        i *= 2;
    }
}
