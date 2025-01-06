use std::io;

fn main() {
    // Helper function to read a floating-point number from the user
    fn read_input(prompt: &str) -> f32 {
        loop {
            println!("{}", prompt);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            // Try to parse the input to a f32
            match input.trim().parse::<f32>() {
                Ok(value) => return value,
                Err(_) => println!("Invalid input. Please enter a valid number."),
            }
        }
    }

    // Read the three sides of the triangle
    let a = read_input("Enter the first edge of the triangle:");
    let b = read_input("Enter the second edge of the triangle:");
    let c = read_input("Enter the third edge of the triangle:");

    // Check if the sides form a valid triangle (Triangle inequality theorem)
    if a + b > c && a + c > b && b + c > a {
        // Calculate the semi-perimeter
        let s = (a + b + c) / 2.0;

        // Calculate the area using Heron's formula
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();

        // Print the result
        println!("Area of the triangle: {:.2}", area);
    } else {
        println!("The given sides do not form a valid triangle.");
    }
}

