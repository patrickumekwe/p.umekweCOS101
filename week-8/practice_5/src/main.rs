fn main() {
    let mut city: Vec<String> = Vec::new();
    city.push(String::from("Abuja"));
    city.push(String::from("Lagos"));
    city.push(String::from("Abia")); //No lagos life for this one yet

    println!("Favorite cities: {:?}", city);
}
