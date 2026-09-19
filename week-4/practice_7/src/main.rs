use std::io;

fn main() {
    println!("while loop time");

    let mut num = String::new();

    println!("Enter a starting number:");
    io::stdin().read_line(&mut num).expect("didnt catch that 😔");
    let mut num: i32 = num.trim().parse().expect("num isnt an integer");

    while num < 10 {
        println!("Current number: {}", num);
        num += 1;
    }

    println!("The loop finally gave up. num is now {}", num);
}
