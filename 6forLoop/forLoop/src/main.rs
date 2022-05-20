// For Loops: used to iterate over a range of elements.

fn main() {
let _x = 1..6;
    // for i in x {
    //     println!("Number is {}", i);
    // }

    let fruits = vec!["orange", "apple", "mango"];
    
    // Enumerate: An iterator that yields the current count and the element during iteration.
    for (index, i) in fruits.iter().enumerate(){
        println!("Fruit number {} is {}", index, i);
    }
}