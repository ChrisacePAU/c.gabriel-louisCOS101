fn main() {
    let k1 = "Yemisi".to_string();
    let k2 = " Shyllon".to_string();
    let k3 = " Museum".to_string();
    let k4 = " of".to_string();
    let k5 = " Art,".to_string();
    let k6 = " PAU".to_string();

    // Using the format macro correctly
    let k7 = format!("{}{}{}{}{}{}", k1, k2, k3, k4, k5, k6);

    // Printing output
    println!("In: {}", k7);
}