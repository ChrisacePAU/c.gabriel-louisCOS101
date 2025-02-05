fn main() f

// mutable array
Let mut colors ["red"

green"

yellow", "white"];

println!("InOriginal array = f:?)", colors);

// mutable slice
Let sliced_colors &mut colors[1..3];

printlni("First slice (:?)", sliced_colors);
// change the value of the original slice at the first index
sliced colors[1] "purple";

println!("Changed slice f:?)", sliced_colors);