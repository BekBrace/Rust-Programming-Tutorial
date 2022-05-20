// Structs are similar to tuples, in that  the elements of a struct can be different types.

struct Ingredients{

    rice: u8,
    crab: u8, 
    nori: u8,
    wasabi: u8

}

fn main() {
   
    let mut makizushi = Ingredients{rice: 50, crab:1, nori:1, wasabi: 1};

    makizushi.rice = 60;

    println!("We need {} gms of rice, {} crab, {} nori, and {} of wasabi",makizushi.rice, makizushi.crab, makizushi.nori, makizushi.wasabi);







}




 // struct User {
    //     active: bool,
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    // }