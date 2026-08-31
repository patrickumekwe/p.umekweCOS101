fn main() {
    let p: f64 = 520000000.0; 
    let r: f64 = 0.10;
    let t: i32 = 5;

    let a = p * (1.0 + r).powi(t);
    let interest = a - p;

    println!("If your principal is: ₦{:.2}", p);
    println!("The total amount you will have after {} years: ₦{:.2}", t, a);
    println!("Compound interest is: ₦{:.2}", interest);
}