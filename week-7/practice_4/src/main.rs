use std::io;

fn add(a: i32, b: i32) {
    println!("The sum of {} and {} is {}", a, b, a + b);
}

fn main() {
    let mut first = String::new();
    let mut second = String::new();

    println!("Enter the first number:");
    io::stdin()
        .read_line(&mut first)
        .expect("failed to read input");
    let a: i32 = first
        .trim()
        .parse()
        .expect("first number must be an integer");

    println!("Enter the second number:");
    io::stdin()
        .read_line(&mut second)
        .expect("failed to read input");
    let b: i32 = second
        .trim()
        .parse()
        .expect("second number must be an integer");

    add(a, b);
}
