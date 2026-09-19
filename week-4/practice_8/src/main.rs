use std::io;

fn main() {
    println!("loop and break");

    let mut x = String::new();

    println!("Enter a number to start from:");
    io::stdin().read_line(&mut x).expect("didnt catch that 😔");
    let mut x: i32 = x.trim().parse().expect("x isnt an integer");

    loop {
        println!("x = {}", x);

        if x == 15 {
            break;
        }

        x += 1;
    }

    println!("We escaped at x = {}", x);
}
