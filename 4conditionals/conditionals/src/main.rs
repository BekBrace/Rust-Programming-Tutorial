// Conditionals : Way to execute certain code based on decision taken / if .. else

fn main() {
    let x = 40;

    if x <= 20 { 
        println!("You are less than 20 y.o.");
    } else if x <= 30 {
        println!("You are less than 30 y.o.");
    } else {
        println!("Your age is {}", x);
    }
}
