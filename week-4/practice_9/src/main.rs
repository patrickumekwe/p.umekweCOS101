use std::io;

fn main() {
    println!("continue is the skip button");

    let mut limit = String::new();

    println!("Enter a limit:");
    io::stdin().read_line(&mut limit).expect("didnt catch that 😔");
    let limit: i32 = limit.trim().parse().expect("limit isnt an integer");

    let mut count = 0;

    for num in 1..=limit {
        if num > 10 {
            continue;
        }

        count += 1;
        println!("Processed number: {}", num);
    }

    println!("Count is {}", count);
}
