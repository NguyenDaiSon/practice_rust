/// This program is to learn how to use `println!` macro
fn main() {
    // Default
    // println!(10);
    // -> error: format argument must be a string literal -> "10"
    println!("Default---------------------------------------");
    println!("10");
    println!("Hello, world!");
    println!("----------------------------------------------");

    //  Formatting: {}
    println!("Formatting------------------------------------");
    println!("{}", "Hello, world!");
    println!("{}, {}", "Hello", "world!");
    println!("----------------------------------------------");

    // Positional arguments {0}, {1},..
    println!("Positional arguments--------------------------");
    println!("{0}, {1}", "Hello", "world!");
    println!("{1}, {0}", "world!", "Hello");
    println!("----------------------------------------------");

    // Named arguments
    println!("Named arguments-------------------------------");
    println!("{greet}, {name}", greet = "Hello", name = "world!");
    println!("{greet}, {name}", name = "world!", greet = "Hello");
    println!("----------------------------------------------");

    // Printing traits
    println!("Traits----------------------------------------");
    println!("10-in-binary={:b}", 10);
    println!("10-in-hex={:x}", 10);
    println!("10-in-octal={:o}", 10);
    println!("----------------------------------------------");

    // Print debug
    println!("Debugs----------------------------------------");
    println!("Tuple={:?}", (10, 20));
    println!("Array={:?}", [10, 20]);
    println!("----------------------------------------------");
}
