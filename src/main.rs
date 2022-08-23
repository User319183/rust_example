
use std::io;



fn main() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("What is your age?");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: i32 = age.trim().parse().expect("Please type a number!");
    println!("Hello, {}. Oh your {} years old?", name.trim(), age);
}