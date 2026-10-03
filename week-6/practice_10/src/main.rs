fn main() {
    let a = 15;
    let b = 25;
    let c = 5;
    let d = 30;
    let is_elder = false;

    let and_result = (a > 10) && (b > 10);
    let or_result = (c > 10) || (d > 10);
    let not_result = !is_elder;

    println!("(a > 10) && (b > 10): {}", and_result);
    println!("(c > 10) || (d > 10): {}", or_result);
    println!("!is_elder: {}", not_result);
}
