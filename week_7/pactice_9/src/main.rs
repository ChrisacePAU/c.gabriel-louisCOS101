fn main() {
    // Define an array with explicit type annotation
    let arr: [i32; 4] = [10, 20, 30, 40];

    println!("Array is: {:?}", arr);
    println!("Array size is: {}", arr.len());

    // Iterate through the array using `iter()`
    for val in arr.iter() {
        println!("Value is: {}", val);
    }
}