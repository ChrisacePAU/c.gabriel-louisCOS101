//declare a structure
struct Employee {
    ceo:String,
    company:String,
    age:u32
}
fn main() {
   //initialize a sructure
   let emp1 = Employee {
    company:String::from("Microsoft Cooperation"),
    ceo:String::from("Sundai Pichai"),
    age:56
   };
   let emp2 = Employee{
    company:String::from("Google Inc."),
    ceo:String::from("Sundai Pichai"),
    age:51
   };
   //pass emp1 and emp2 to display()
   display(emp1);
   display(emp2);
}
//fetch values of specific structure fields using the 
// operator and print it to the console
fn display( emp:Employee){
    println!("Name is :{} \n, company is {} \n, age is {}",emp.ceo,emp.company,emp.age);
}