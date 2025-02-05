use std::io;

fn main() {
    // Vector containing characters
    let v = vec!['C', '0', 'M', 'P', 'U', 'T', 'E', 'R'];

    // Reading input from user
    let mut input1 = String::new();
    println!("Enter an index value between (0 - 7):");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    // Index is the non-negative value, smaller than the size of the vector
    let index: usize = input1.trim().parse().expect("Invalid input");

    // Getting the value at the given index
    let ch: char = v[index];

    // Printing the character at the given index
    println!("{} is the character at index [{}]", ch, index);
}