fn main() {
    println!("Mutable slice");

    let mut colors = vec!["red", "black", "blue", "yellow"];

    {
        let sliced_colors = &mut colors[1..2]; // my favourite
        sliced_colors[1] = "brown";
    }

    println!("Updated colors: {:?}", colors);
}
