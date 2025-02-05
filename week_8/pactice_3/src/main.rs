fn main()use std::io;

// Method to print the value
fn value(n: Option<&char>) {
    match n {
        Some(val) => println!("Element of vector: {}", val),
        None => println!("Invalid index!"),
    }
}

fn main() {
    // Vector containing characters
    let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'I', 'A', 'N'];

    // Reading input from the user
    let mut input1 = String::new();
    println!("Enter an index value between (0 - 8):");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    // Parse input as a non-negative value smaller than the size of the vector
    let index: usize = input1.trim().parse().expect("Invalid input");

    // Getting the value at the given index
    let ch: Option<&char> = v.get(index);

    // Calling the value function to print the element
    value(ch);
}