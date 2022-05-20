// References [ Borrowing ] : a way to refere to a variable but without taking the ownership of the variable itself. 

fn main() {
    
    let mut number:i8 = 77;

    let numRef = &mut number;

    *numRef += 1;

    println!("REF: Number is {}", numRef );

}
