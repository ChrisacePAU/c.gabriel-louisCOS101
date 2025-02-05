fn main() {
    let a: i32 = 2;
    let b: i32 = 3;

    // Bit presentation:
    // a = 2  ->  10 (binary)
    // b = 3  ->  11 (binary)

    let mut result: i32;

    result = a & b;
    println!("(a & b) => {}", result