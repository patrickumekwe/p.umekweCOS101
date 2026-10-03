fn main() {
    let fullname = " Pan-Atlantic Universe "; //lol

    println!("Original text: '{}'", fullname);
    println!("Length before trim: {}", fullname.len());
    println!("Length after trim: {}", fullname.trim().len());
    println!("Trimmed text: '{}'", fullname.trim());
}
