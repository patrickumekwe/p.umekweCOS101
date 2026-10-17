fn main() {
    println!("Destructuring");

    let x = (16, true, 5.0);
    let (age, is_male, cgpa) = x;

    println!("Age is {}", age);
    println!("is_male is {}", is_male);
    println!("cgpa is {}", cgpa);
}
