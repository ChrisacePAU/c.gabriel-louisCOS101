fn main() {
    // Initialize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    // Print the original tuple
    println!("Original tuple: {:?}", mountain_heights);

    // Change 3rd and 4th elements of a mutable tuple
    mountain_heights.2 = "Lhotse";  // Changing the name of the third element
    mountain_heights.3 = 8516;      // Changing the height of the fourth element

    // Print the changed tuple
    println!("Changed tuple: {:?}", mountain_heights);
}