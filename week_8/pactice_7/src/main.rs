fn main() {
    // Initialization of tuple with data type
    let datatype_tuple: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tuple contents: {:?}", datatype_tuple);

    // Initialization of tuple without specifying data type (Rust infers the type)
    let no_datatype_tuple = ("Rust", "fun", 100);
    println!("Tuple contents: {:?}", no_datatype_tuple);

    // Accessing tuple element at index 0
    println!("Value at Index 0: {}", datatype_tuple.0);

    // Accessing tuple element at index 1
    println!("Value at Index 1: {}", datatype_tuple.1);

    // Accessing tuple element at index 2
    println!("Value at Index 2: {}", datatype_tuple.2);
}