use std::io::Write;
use std::fs::File;

fn main() {
    println!("fILE ");
    let name = "Patrick\n";
    let age = "12";

    let mut file = File::create("/home/PCU/Documents/p.umekweCOS101/week-9/practice_1/src/data,txt").expect("Failed create procees"); //Create...... crate?
    file.write_all("Asdf\n".as_bytes()).expect("Write Failed");
    file.write_all(name.as_bytes()).expect("Write Failed");
    file.write_all(age.as_bytes()).expect("Write Failed");
    println!("{:#?}", age.as_bytes());
}
