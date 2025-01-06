// RRUST PROGRAM FOR CALCULATIONS
use std::io;
fn main() {
	println!("Good Day professor");
	println!("I can help you with");
	help_with();
	 fn Trapezium_solution(){
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();

	  println!("Enter base1:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let base1:f32 = input2.trim().parse().expect("Not a vald number");

      println!("Enter base2:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let base2:f32 = input3.trim().parse().expect("Not a vald number");

    println!("Enter height: ");
    io::stdin().read_line(&mut input4).expect("Not a valid number");
    let height:f32 = input4.trim().parse().expect("Not a valid number");

    let area = 0.5 * (base1 + base2) * height;
    println!("The Area of the trapezium {}",area);
    }
		
}

	fn help_with(){
		println!("Area of Trapezium");
		println!("yes or no");
       let mut input1 = String::new();
       io::stdin().read_line(&mut input1).expect("Failed to read input");
      
         let mut input5 = String::new();
       io::stdin().read_line(&mut input5 ).expect("Failed to read input");
      
       

    }

    
   


    

