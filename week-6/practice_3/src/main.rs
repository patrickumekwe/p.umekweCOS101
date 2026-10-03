fn main() {
    let name1 = "Ayomide";
    let renamed_name = name1.replace("Ayomide", "Adebare");

    let faculty = String::from("Faculty of Science");
    let school = faculty.replace("Faculty", "School");

    println!("Original name: {}", name1);
    println!("Renamed name: {}", renamed_name);
    println!("Original faculty: Faculty of Science");
    println!("Renamed faculty: {}", school);
}
