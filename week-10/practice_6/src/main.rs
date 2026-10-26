fn add_one(e:&mut i32){
    return *e+=1;
}

fn main() {
    let mut i = 3;
    add_one(&mut i);
    println!("{}", i); // prints 5
    // i can write notes now that i have eaten
}
