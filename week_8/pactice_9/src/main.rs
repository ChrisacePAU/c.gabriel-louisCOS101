fn main() {
    let b: (i32, bool, f64) = (110, true, 10.9);
    print(b);
}

// Pass the tuple as a parameter
fn print(x: (i32, bool, f64)) {
    println!("Inside print method");
    println!("{:?}", x); // Use {:?} for tuple printing
}