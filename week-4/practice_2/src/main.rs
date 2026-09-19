use std::io;

fn main() {
    println!("Triangle area calculator");

    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter the first side:");
    io::stdin().read_line(&mut a).expect("didnt catch that 😔");
    let a: f32 = a.trim().parse().expect("a isnt a number");

    println!("Enter the second side:");
    io::stdin().read_line(&mut b).expect("didnt catch that 😔");
    let b: f32 = b.trim().parse().expect("b isnt a number");

    println!("Enter the third side:");
    io::stdin().read_line(&mut c).expect("didnt catch that 😔");
    let c: f32 = c.trim().parse().expect("c isnt a number");
     println!("Enter the fourth side:");
     println!("What do you mean there is no fourth side? \n fine ");
    let s: f32 = (a + b + c) / 2.0;
    let area = (s * (s - a) * (s - b) * (s - c)).sqrt();

    println!("Your triangle area is {}", area);
}
