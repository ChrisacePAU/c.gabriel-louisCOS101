use std::io;

fn main() {
    // Create an empty vector "City"
    let mut city: Vec<String> = Vec::new();

    // Print City Vector
    println!("The City vector has {} elements.", city.len());

    // Ask how many cities the user wants to enter
    let mut input1 = String::new();
    println!("How many cities do you want to enter?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    // Parse the number of cities
    let city_num: i32 = input1.trim().parse().expect("Invalid input");

    // Loop to get cities from the user
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter City {}: ", count + 1);
        io::stdin().read_line(&mut input2).expect("Failed to read input");

        // Add the new city to the vector
        let new_city: String = input2.trim().to_string();
        city.push(new_city);
    }

    // Print the cities entered
    println!("\nYour preferred cities are:");
    let mut count = 1;
    for city_name in &city {
        println!("{}: {}", count, city_name);
        count += 1;
    }
}