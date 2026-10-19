
use std::io::Read;
use std::fs::File;

fn main() {
    println!("fILE ");
    let mut file = File::open("data.txt").expect("Failed create procees"); //Create...... crate?

    let mut cont = String::new();
     file.read_to_string(&mut cont).unwrap();
    println!(" {}" ,cont);
}
