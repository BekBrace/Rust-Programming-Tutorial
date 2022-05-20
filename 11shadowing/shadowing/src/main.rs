// Shadowing: Language Feature that allows you to re-declare a variable in the same scope, using the same name.

fn main() {
    let mut number: u32 = 100;

    {
        let number = 150;
    }

    println!("Number is {}", number);

    print_type_of(&number);

    fn print_type_of<T>(_: &T) {
        println!("Type of number is : {} ", std::any::type_name::<T>());
    }

    let number: &str = "Number!";
    println!("number is {}", number );
    print_type_of(&number);

}
