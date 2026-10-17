fn print(x: (i32, bool, f64)) {
    println!("Tuple: {:?}", x);
}

fn main() {
    println!("Passing a tuple");

    let b = (2, false, 3.5);
    print(b);
}
