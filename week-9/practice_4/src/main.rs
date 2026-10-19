use std::fs::OpenOptions;
use std::fs::Write;

fn main(){
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("asdf");
    file.write_all("asdf".as_bytes()).expect("asdf write fail");
    println!("Failed);
}
