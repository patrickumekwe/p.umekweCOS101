fn main() {
    let n1 = String::from("Hello");
    let n2 = String::from("World");
    let n3 = String::from("!");

    let n4 = n1 + &n2 + &n3;
    println!("Concatenated string: {}", n4);
}
