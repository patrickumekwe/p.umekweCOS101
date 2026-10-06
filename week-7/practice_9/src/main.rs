fn main() {
    let city_arr: [&str; 5] = ["Abuja", "Lagos", "Ibadan", "Kano", "Enugu"];
    //  did you really read all of my notes
    for val in city_arr.iter() {
        println!("{}", val);
    }
}
