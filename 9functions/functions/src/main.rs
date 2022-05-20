// Functions 

// functions outside of Main function scope
fn add_num(x: i64, y: i64) {
    let z = x + y;
    println!("The addition of {} + {} = {}", x,y,z);
}
fn sub_num(x: f64, y: f64) {
    let z = x - y;
    println!("The sub of {} - {} = {}", x,y,z);
}
fn mult_num(x: i64, y: i64) {
    let z = x * y;
    println!("The mult of {} X {} = {}", x,y,z);
}
fn div_num(x: i64, y: i64) {
    let z = x / y;
    println!("The division of {} / {} = {}", x,y,z);
}


fn main() {

    // Calling the add_num function
    add_num(121 , 78); 
    // Calling the sub_num function
    sub_num(78.2 , 32.4);
    // Calling the mult_num function 
    mult_num(121 , 78); 
    // Calling the div_num function
    div_num(121 , 78);
    
    // Functions inside main function scope
    fn lower() {
        let _z_string = "BANANARAMA";
        println!("The lower case of The String is {}", _z_string.to_ascii_lowercase());
    }
    fn len() {
        let _z_string = "BANANARAMA";
        println!("The length of Bananarama is {}", _z_string.len());
    }
    lower();
    len();
}
