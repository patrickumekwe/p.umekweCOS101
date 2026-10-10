use std::io;

fn trapezium() -> f64 {
    let mut input = String::new();

    println!("Enter height:");
    io::stdin().read_line(&mut input).unwrap();
    let height: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter first base:");
    io::stdin().read_line(&mut input).unwrap();
    let base1: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter second base:");
    io::stdin().read_line(&mut input).unwrap();
    let base2: f64 = input.trim().parse().unwrap();

    height / 2.0 * (base1 + base2)
}

fn rhombus() -> f64 {
    let mut input = String::new();

    println!("Enter first diagonal:");
    io::stdin().read_line(&mut input).unwrap();
    let d1: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter second diagonal:");
    io::stdin().read_line(&mut input).unwrap();
    let d2: f64 = input.trim().parse().unwrap();

    0.5 * d1 * d2
}

fn parallelogram() -> f64 {
    let mut input = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut input).unwrap();
    let base: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter altitude (height):");
    io::stdin().read_line(&mut input).unwrap();
    let altitude: f64 = input.trim().parse().unwrap();

    base * altitude
}

fn cube() -> f64 {
    let mut input = String::new();

    println!("Enter side length:");
    io::stdin().read_line(&mut input).unwrap();
    let side: f64 = input.trim().parse().unwrap();

    6.0 * side * side
}

fn cylinder() -> f64 {
    let mut input = String::new();

    println!("Enter radius:");
    io::stdin().read_line(&mut input).unwrap();
    let radius: f64 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).unwrap();
    let height: f64 = input.trim().parse().unwrap();

    std::f64::consts::PI * radius * radius * height
}

fn main() {
    println!("SHAPE CALCULATOR");
    println!("1. Trapezium");
    println!("2. Rhombus");
    println!("3. Parallelogram");
    println!("4. Cube");
    println!("5. Cylinder");

    let mut choice = String::new();

    println!("Choose a shape:");
    io::stdin().read_line(&mut choice).unwrap();

    let choice: u32 = choice.trim().parse().unwrap();

    match choice {
        1 => println!("Trapezium area: {:.2}", trapezium()),
        2 => println!("Rhombus area: {:.2}", rhombus()),
        3 => println!("Parallelogram area: {:.2}", parallelogram()),
        4 => println!("Cube surface area: {:.2}", cube()),
        5 => println!("Cylinder volume: {:.2}", cylinder()),
        _ => println!("That option doesn't exist."),
    }
}
