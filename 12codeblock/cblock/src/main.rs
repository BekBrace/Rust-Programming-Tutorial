// Code Block : Block of code betweeen curly braces yet still have access to data stored outside the code block.

fn main() {
    
    let surname: &str = "Brace";

    { 
    //    inner scope
        let name: &str = "Bek"; 
        println!("Name is {} and surname is {} ", name, surname );

    }

   


}
