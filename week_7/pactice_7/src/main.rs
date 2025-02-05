fn main() {
    // Array with explicit integer data type
    let arr1: [i32; 4] = [10, 20, 30, 40];
    println!("In Array with data type");
    println!("Array is: {:?}", arr1);
    println!("Array size is: {}", arr1.len());

    // Array with inferred float data type
    let arr2 = [10.4, 20.7, 30.4, 40.9, 51.2, 72.2];
    println!("\nIn Array without explicit data type");
    println!("Array is: {:?}", arr2);
    println!("Array size is: {}", arr2.len());

    // Array with default values (-1 repeated 8 times)
    let arr3: [i32; 8] = [-1; 8];
    println!("\nIn Array with default values");
    println!("Array is: {:?}", arr3);
    println!("Array size is: {}", arr3.len());
}