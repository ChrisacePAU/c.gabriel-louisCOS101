fn main() {
    // Define an array of city names
    let city_arr: [&str; 5] = ["Abuja", "Portharcourt", "Maiduguri", "Kano", "Lagos"];
    
    println!("Array is: {:?}", city_arr);
    println!("Array size is: {}", city_arr.len());

    // Loop through the array using an index
    for index in 0..city_arr.len() {
        println!("City at index {} is: {}", index, city_arr[index]);
    }
}