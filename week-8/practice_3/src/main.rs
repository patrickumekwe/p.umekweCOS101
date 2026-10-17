fn main() {
    println!("The safer way"); //according to my lectures

    let v = vec!['U', 'S', 'T', 'A', 'R'];

    println!("Value at index 2: {:?}", v.get(2));
    println!("Value at index 8: {:?}", v.get(8));
}
