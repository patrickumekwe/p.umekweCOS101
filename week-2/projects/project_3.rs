fn main() {
    let p:f64 = 210000.0; 
    let r: f64 = 0.05;
    let t: i32 = 3;

    let a = p * (1.0 - r).powi(t);
    let interest = a - p;

    println!("If your original price of the TV was: ₦{:.2}", p);
    println!("The value after {} years:₦{:.2}", t, a);
    println!("It depreciated by:₦{:.2}",interest);
}