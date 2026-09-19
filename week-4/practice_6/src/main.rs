use std::io;

fn main() {
    println!("Counting with for");

    let mut lower = String::new();
    let mut upper = String::new();

    println!("Enter the lower bound:");
    io::stdin().read_line(&mut lower).expect("didnt catch that 😔");
    let lower: i32 = lower.trim().parse().expect("lower bound isnt an integer");

    println!("Enter the upper bound:");
    io::stdin().read_line(&mut upper).expect("didnt catch that 😔");
    let upper: i32 = upper.trim().parse().expect("upper bound isnt an integer");

    for x in lower..upper {
        println!("Current count: {}", x);
    }
}
