fn main() {
    let mut v: Vec<i64> = Vec::new();
    println!("Empty vector length: {}", v.len());

    v.push(10);
    v.push(20);
    v.push(30);

    let w = vec!["Zion", "Patrick", "Kayode", "Efe"];
    println!("vector length: {}", w.len());
    println!("First value: {}", w[0]);
    println!("All values: {:?}", w);
}
