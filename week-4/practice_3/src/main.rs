use std::io;

fn main() {
    println!("Triangle area check");

    let mut base = String::new();
    let mut height = String::new();

    println!("Enter the base:");
    io::stdin().read_line(&mut base).expect("didnt catch that 😔");
    let base: f32 = base.trim().parse().expect("base isnt a number");

    println!("Enter the height:");
    io::stdin().read_line(&mut height).expect("didnt catch that 😔");
    let height: f32 = height.trim().parse().expect("height isnt a number");

    if base > 0.0 {
        let area = (base * height) / 2.0;
        println!("Area is {}", area);
    } else {
        println!("Well I never \t hmph");
    }
}
