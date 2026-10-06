use std::io;

fn checker() {
    let mut input = String::new();

    println!("Enter a character:");
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    //match mogs if/else
    match input.trim().chars().next() {
        Some(ch) if ch >= '0' && ch <= '9' => println!("{} is a digit", ch),
        Some(ch) => println!("{} is not a digit", ch),
        // you can use _ i know i was suprised too!!
        _ => println!("No character was entered"),
    }
}

fn main() {
    println!("Character checker");
    checker();
}
