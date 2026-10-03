fn main() {
    let mut school = String::from("School of Science");
    school.push_str(" and Technology");

    let fullname = school.clone();
    println!("School: {}", school);
    println!("Full name length: {} bytes", fullname.len());
}
