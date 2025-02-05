use std::io;

fn main() {
    // Define CO2 emission rates (kg of CO2 per hour of use)
    const MOBILE_CO2_PER_HOUR: f32 = 0.018; // 18g = 0.018kg
    const LAPTOP_CO2_PER_HOUR: f32 = 0.050; // 50g = 0.050kg

    // Welcome message
    println!("Welcome to the Carbon Footprint Calculator!");

    // Get the number of mobile devices
    println!("Enter the number of mobile devices in your group:");
    let mobile_count = get_user_input().parse::<u32>().unwrap_or(0);

    // Get the daily usage of mobile devices in hours
    println!("Enter the average daily usage of mobile devices (in hours):");
    let mobile_usage_hours = get_user_input().parse::<f32>().unwrap_or(0.0);

    // Get the number of laptops
    println!("Enter the number of laptops in your group:");
    let laptop_count = get_user_input().parse::<u32>().unwrap_or(0);

    // Get the daily usage of laptops in hours
    println!("Enter the average daily usage of laptops (in hours):");
    let laptop_usage_hours = get_user_input().parse::<f32>().unwrap_or(0.0);

    // Calculate CO2 emissions
    let mobile_emission = (mobile_count as f32) * mobile_usage_hours * MOBILE_CO2_PER_HOUR;
    let laptop_emission = (laptop_count as f32) * laptop_usage_hours * LAPTOP_CO2_PER_HOUR;
    let total_emission = mobile_emission + laptop_emission;

    // Display the results
    println!("\n=== Carbon Footprint Report ===");
    println!("Mobile devices' CO2 emissions: {:.2} kg/day", mobile_emission);
    println!("Laptops' CO2 emissions: {:.2} kg/day", laptop_emission);
    println!("Total CO2 emissions: {:.2} kg/day", total_emission);

    // Preventive measures
    println!("\n=== Preventive Measures ===");
    println!("1. Turn off devices when not in use.");
    println!("2. Use energy-efficient chargers and devices.");
    println!("3. Reduce screen brightness and enable power-saving modes.");
    println!("4. Encourage group members to share devices where possible.");
}

// Helper function to get user input
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}