fn main() {
    println!("Slice time");

    let numbers = [10, 20, 30, 46, 50];

    let slice1 = &numbers[1..2];
    let slice2 = &numbers[..3];
    let slice3 = &numbers[1..4];
    let slice4 = &numbers[..5];

    println!("slice1: {:?}", slice1);
    println!("slice2: {:?}", slice2);
    println!("slice3: {:?}", slice3);
    println!("slice4: {:?}", slice4);
}
