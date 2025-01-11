fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan ATlantic University";
    let addr:&str = "Km 52 Lakki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}",name);
    println!("University: {}, \nAddress: {}",uni,addr);


    let department:&'static str = "Computer Science";
    let school:&'static str = "School of science and Technology";
    println!("Department; {}, \nSchool: {}",department,school);
}