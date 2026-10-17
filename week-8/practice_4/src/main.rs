fn main() {
    println!("Name and age");

    let name = vec!["Patrick", "Zion", "Dieko"];
    let age = vec![16, 19, 20];

    for i in 0..age.len() {
        println!("{} is {} years old", name[i], age[i]);
    }
}
