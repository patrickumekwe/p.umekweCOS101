use std::io;

fn main() {
    println!("Height ladder");

    let mut height = String::new();

    println!("Enter your height:");
    io::stdin().read_line(&mut height).expect("didnt catch that 😔");
    let height: f32 = height.trim().parse().expect("height isnt a number");

    if height >= 150.0 && height <= 170.0 {
        println!("Average height");
    } else if height > 170.0 && height <= 195.0 {
        println!("Tall height");
    } else if height < 150.0 {
        println!("Short height");
    } else {
        println!("Abnormal height");
    }
}
