fn main() f
I an array of numbers
let numbers - [1, 2, 3, 4, 5];

Practice 11: week-8/practice_11/src/main.rs

println!("Original array - (:?)", numbers);

// create a
slice of 2nd and 3rd element
Let slicel = &numbers[1..3];
println!("2nd and 3rd elements sliced - (:?)", slicel);

// omit the start index
let slice2 = &numbers[..3];
// This means the slice starts from index 0 and goes up to index 3 (exclusive)
println!("index 0 to index 3 sliceed = (:?)", slice2);

// omit the end index
Let slice3 = &numbers[2..];
// This means the slice starts from index 2
and goes up to index 5 (exclusive)
printLn!("index 2 to index 5 sliced = (:?)", slice3);

// omit the start index and the end index
// reference the whole array
Let slice4 = &numbers[..];
// This means the slice starts from index 0 and goes up to index 5 (exclusive).
printLn!("index 0 to index 5 sliced - (:?)", slice4);