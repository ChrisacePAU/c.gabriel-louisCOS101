fn main() {
    let v = vec![20,40,60,80];
    // vector v owns the object in heap

    let v2 = v;
    let v2_retrn = display(v2);
    println!("In mai{:?}",v); 

}

fn display(v:Vec<i32>)->Vec<i32> {
    //returning same vector 
    println!("inside display{:?}",v);
    return v;
}