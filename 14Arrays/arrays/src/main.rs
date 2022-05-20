// Arrays: Sequence of values inside square brackets that you can access through variable name 
// (arrays in JS/c/c++/java/c#, Lists in Python)
// [Homogenous elements]

fn main() {
    let num_Array = [1,2,3,4,5];
    let str_Array = ["Python", "Rust", "Java", "Go"];
    println!("Num 0 is: {}", num_Array[0]);
    println!("Num 1 is: {}", num_Array[1]);
    println!("Str 3 is: {}", str_Array[3]);
    println!("Str 1 is: {}", str_Array[1]);

    // Printing all items with for loop
    for num in num_Array{
        println!("{}", num );
    }
    for i in str_Array{
        println!("Language: {}", i );
    }

    for num in 0..num_Array.len() {
        println!("{}", num );
    }
}
