fn main() {
    let mut num: i32 = 0; // Initialize `num` before use
    mutate_num_to_zero(&mut num);
    println!("The value of num is: {}", num);
}

fn mutate_num_to_zero(param_num: &mut i32) {
    *param_num = 0; // Dereference to modify the value
    println!("param_num value is: {}", param_num);
}