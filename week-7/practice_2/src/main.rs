use std::io;

fn checker() {
    let mut a = String::new();

    println!("Enter a character:");
    io::stdin().read_line(&mut a).expect("didnt catch that 😔");

    match a.trim().chars().next() {
        Some(ch) if ch >= '0' && ch <= '9' => println!("{} is a digit", ch),
        Some(ch) => println!("{} is not a digit", ch),
        None => println!("No character was entered"),
    }
}

fn main() {
    println!("Character checker");
    checker();
}
