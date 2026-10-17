fn main() {
    println!("Tuple reading");

    let t: (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("t.0 = {}", t.0);
    println!("t.1 = {}", t.1);
    println!("t.2 = {}", t.2);
}
