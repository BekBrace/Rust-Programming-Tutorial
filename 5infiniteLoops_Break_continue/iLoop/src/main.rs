// INFINITE LOOPS / BREAK / CONTINUE

fn main() {
    let mut n = 0;
    loop {
        // n+=1; incrementing n by 1
        n = n + 1; 
        // println!("The value of n is {}", n);

        // Continue
        if n == 13 {
            continue;
        }
        // Break
        if n > 15 {
            break;
        }
        println!("The value of n is {}", n);
    }
}
