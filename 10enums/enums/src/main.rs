//enums: A type that represents data that is one of several possible variants.

enum Compass {
    North, 
    South,
    East, 
    West 
}

fn main() {
    let soldier1:Compass = Compass::North;
    let soldier2:Compass = Compass::South;
    let soldier3:Compass = Compass::East;
    let soldier4:Compass = Compass::West;

    match soldier1 {

        Compass::North => println!("Soldier1 : To Winterfell!"),
        Compass::South => println!("Soldier2 : To Westros!"),
        Compass::East => println!("Soldier3 : To King's Landing!"),
        Compass::West => println!("Soldier4 : To Casterly Rock!"),

    }
    match soldier2 {

        Compass::North => println!("Soldier1 : To Winterfell!"),
        Compass::South => println!("Soldier2 : To Westros!"),
        Compass::East => println!("Soldier3 : To King's Landing!"),
        Compass::West => println!("Soldier4 : To Casterly Rock!"),

    }
    match soldier3 {

        Compass::North => println!("Soldier1 : To Winterfell!"),
        Compass::South => println!("Soldier2 : To Westros!"),
        Compass::East => println!("Soldier3 : To King's Landing!"),
        Compass::West => println!("Soldier4 : To Casterly Rock!"),

    }
    match soldier4 {

        Compass::North => println!("Soldier1 : To Winterfell!"),
        Compass::South => println!("Soldier2 : To Westros!"),
        Compass::East => println!("Soldier3 : To King's Landing!"),
        Compass::West => println!("Soldier4 : To Casterly Rock!"),

    }

    
}
