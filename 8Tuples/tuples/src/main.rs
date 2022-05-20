//Tuples

fn main() {
    // String tuple
    let sTuple = ("rice", "crab", "fish", "salad");
    // Integer Tuple
    let iTuple = (1,2,3,4,5);
    // Mixed Tuple
    let mixed_Tuple = ("Smith", "Laura", true, 5.2, 100, 'x');
    println!("The Second name is {}, and the integer is {}, while the Boolean value is {} ", mixed_Tuple.1, mixed_Tuple.4, mixed_Tuple.2);

    // Nested Tuple
    let N_mixed_Tuple = ("Smith", "Laura", true, 5.2, 100, 'x', (10,20,30));
    println!("first value of nested tuple is {}, the second is {}, and the third is {}", (N_mixed_Tuple.6).0, (N_mixed_Tuple.6).1, (N_mixed_Tuple.6).2 );
}
