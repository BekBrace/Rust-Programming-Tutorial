// While Loop: the while loop loops through a block of code as long as a specified condition is true.

fn main() {
    let mut number: i32 = 1;

    // while number <= 20 {
    //     println!("The number is {}", number);
    //     // number = number + 1;
    //     number +=1;
    // }
    
    while number <= 40 {
        if number % 2 == 0 {
            println! ("Number {} is Even", number);
        } else {
            println! ("Number {} is Odd", number);
        }
        number +=1 ;
    }
}

