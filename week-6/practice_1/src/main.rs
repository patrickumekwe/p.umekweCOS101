fn main() {
    let name = "Aisha Lawal";
    let uni: &str = "Pan-Atlantic University";
    let address = "KM 10, Lekki-Epe Expressway";
    let department: &'static str = "Computer Science";
    let school = "School of Science";

    println!("Name: {}\nUniversity: {}\nAddress: {}\nDepartment: {}\nSchool: {}",
        name, uni, address, department, school);
}
