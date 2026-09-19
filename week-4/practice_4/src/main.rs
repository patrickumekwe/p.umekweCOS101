use std::io;

fn main() {
    println!("Party gate");

    let mut name = String::new();
    let mut age = String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("didnt catch that 😔");

    println!("Enter your age:");
    io::stdin().read_line(&mut age).expect("didnt catch that 😔");
    let age: u32 = age.trim().parse().expect("age isnt an integer");

    if age >= 18 {
        println!("Welcome to the party, {}", name.trim());
    } else {
        println!("Oops, you are not of age to enter the party.");
    }
}
