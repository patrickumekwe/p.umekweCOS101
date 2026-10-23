use std::io::Write;
use std::fs::File;

fn main() {
   let students: [(&str, &str, &str, u32); 5] = [
    ("Oluchi Mordi",   "ACC10211111", "Accounting", 300),
    ("Adams Aliyu",    "ECO10110101", "Economics",  100),
    ("Shania Bolade",  "CSC10328828", "Computer",   200),
    ("Adekunle Gold",  "EEE11020202", "Electrical", 200),
    ("Blanca Edemoh",  "MEE10202001", "Mechanical", 100),
];
    let mut list = String::from("Name,Matric No, Dept, Level\n");
    let mut file = File::create("data.csv").expect("Failed create procees");
    
    for i in 0..students.len(){
        list.push_str(&format!("{},", students[i].0));
          list.push_str(&format!("{},", students[i].1));
            list.push_str(&format!("{},", students[i].2));
           list.push_str(&format!("{},\n", students[i].3));
        
        
    }
    //done and dusted
    

    file.write_all(list.as_bytes()).expect("asdfghjk");

     
}
