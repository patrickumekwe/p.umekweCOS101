use std::io::Read;

// Opens a file and prints everything in it
fn show_file(filename: &str) {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

// One function per role
fn administrator() {
    show_file("globacom_schema.sql");
}

fn project_manager() {
    show_file("project_tb.sql");
}

fn employee() {
    show_file("staff_tb.sql");
}

fn customer() {
    show_file("customer_tb.sql");
}

fn vendor() {
    show_file("dataplan_tb.sql");
}

fn main() {
    println!("Who is asking?");
    println!("administrator, project manager, employee, customer, vendor");

    let mut role = String::new();
    std::io::stdin().read_line(&mut role).unwrap();
    let role = role.trim().to_lowercase();

    if role == "administrator" {
        administrator();
    } else if role == "project manager" {
        project_manager();
    } else if role == "employee" {
        employee();
    } else if role == "customer" {
        customer();
    } else if role == "vendor" {
        vendor();
    } else {
        println!("Unknown role");
    }
}
