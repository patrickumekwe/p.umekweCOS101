fn main() {
    println!("Mutable tuple");

    let mut mountain_heights = ("Everest", 8848, "Lhotse");
    println!("Before: {:?}", mountain_heights);

    mountain_heights.2 = "K2";
    println!("After: {:?}", mountain_heights);
}
