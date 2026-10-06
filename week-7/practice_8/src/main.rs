fn main() {
    let city_arr: [&str; 5] = ["Abuja", "Lagos", "Ibadan", "Kano", "Enugu"];
    // i have 700b in lagos life btw
    for index in 0..5 {
        println!("City {}: {}", index, city_arr[index]);
    }
}
