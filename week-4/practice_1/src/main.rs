use std::io;

fn main() {
    println!("Student info time");

    let mut name = String::new();
    let mut age = String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("didnt catch that 😔");

    println!("Enter your age:");
    io::stdin().read_line(&mut age).expect("didnt catch that 😔");
    let age: u32 = age.trim().parse().expect("age isnt an integer");

    println!("Your name is {}", name.trim());
    println!("Your age is {}", age);
}
