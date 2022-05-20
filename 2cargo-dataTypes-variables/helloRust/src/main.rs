#![allow(non_snake_case)]

// single line comment
/*
 * multiple
 * line
 * comments
 */

// Import data types
use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64}; 
use std::io::stdin;


// Main function
fn main() {
    println!("Hello, Rust Again 🦀!");

    //Immutable Integers
    let num: i8 = 10; // i32 - immutable by default
    println!("the number is {}", num);
    
    // String 
    let name: &str = "Bek";
    let sur_name: &str = "Brace";
    println!("My name is {} {}", name, sur_name);

    // Mutable integers
    let mut num = 100;
    println!("the number is {}", num);

    println!("Max i8 {}", i8::MAX);
    println!("Min i8 {}", i8::MIN);
    println!("Max i16 {}", i16::MAX);
    println!("Min i16 {}", i16::MIN);
    println!("Max i32 {}", i32::MAX);
    println!("Min i32 {}", i32::MIN);
    println!("Max i64 {}", i64::MAX);
    println!("Min i64 {}", i64::MIN);
    println!("Max isize {}", isize::MAX);
    println!("Min isize {}", isize::MIN);
    println!("Max usize {}", usize::MAX);
    println!("Min usize {}", usize::MIN);
    println!("Max f32 {}", f32::MAX);
    println!("Min f32 {}", f32::MIN);
    println!("Max f64 {}", f64::MAX);
    println!("Min f64 {}", f64::MIN);

    // Booleans
    let _is_valid: bool = true; // false
    println!("This is a Boolean: {}", _is_valid );


    // Characters
    let one_char: char = 'a'; // single quotes not double

    // place variable in placeholders
    println!("My name is {} {}, and my fav char is {}", name, sur_name, one_char );


    // Float 
    let _the_float: f32 = 2.5;
    println!("This is a Float :  {}", _the_float);

    // define multiple variables on multiple lines
    let(country, capital) = ("France", "Paris");
    println!("{},  {} ", country, capital); 

     //Operators For Math Calculation
     println!("20 + 4 = {}", 20 + 4);
     println!("20 - 4 = {}", 20 - 4);
     println!("20 * 4 = {}", 20 * 4);
     println!("20 / 4 = {}", 20 / 4);
     println!("20 % 4 = {}", 20 % 4);
}
    
