fn main() {
    let value_1 = "Hello";
    let value_2 = "World";

    // use Display
    println!("{} {}", value_1, value_2); // default by placeholders
    println!("{0} {1}", value_1, value_2); // by indexes
    println!("{hello} {world}", hello = value_1, world = value_2); // by names
    println!("value_1='{}' value_2='{}'", value_1, value_2); // custom names manually

    // use Debug
    println!("{:?} {:?}", value_1, value_2);
    // use Debug and pretty-print
    println!("{:#?} {:#?}", value_1, value_2);
}
