fn main() {
    // Rust's compiler automatically determined `&str` type,
    // so no need to declare it.
    let value_1 = "Hello";
    let value_2 = "World";

    // Simple write to stdout,
    // with new line, and space between printed objects.
    println!("{} {}", value_1, value_2);
    // Write to stdout (change space between objects to "_").
    println!("{}_{}", value_1, value_2);
    // Simple write to stdout (without new line).
    print!("{} {}", value_1, value_2);
}
