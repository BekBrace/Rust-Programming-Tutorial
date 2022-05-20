// Constants : variables cannot be changed - mutability not applied

const _THE_CONST_INT: u8 = 10;
const _THE_CONST_STR: &str = "Cairo";


fn main() {
    // let name: &str = "rust"; you cannot declare variables in global scope
    println!("The integer is : {}, and the city is {}", _THE_CONST_INT, _THE_CONST_STR );
    // _THE_CONST_INT = 20;
}