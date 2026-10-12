use std::io;

fn pick(department: u8) {
    let off_lvl = [
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];

    let aca_lvl = [
        "None",
        "Research Assistant",
        "PhD Candidate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];

    let law_lvl = [
        "Paralegal",
        "Junior Associate",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];

    let tea_lvl = [
        "Placement",
        "Classroom Teacher",
        "Senior Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal",
    ];

    let mut input = String::new();

    println!("Enter years of experience:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let yrs: u16 = match input.trim().parse() {
        Ok(value) => value,
        _ => {
            println!("Invalid years of experience.");
            return;
        }
    };

    let positions = match department {
        1 => &off_lvl,
        2 => &aca_lvl,
        3 => &law_lvl,
        4 => &tea_lvl,
        _ => {
            println!("Invalid department.");
            return;
        }
    };

    let (index, aps) = match yrs {
        0..=2 => (0, "APS 1-2"),
        3..=4 => (1, "APS 3-5"),
        5..=7 => (2, "APS 5-8"),
        8..=9 => (3, "EL1 8-10"),
        10..=12 => (4, "EL2 10-13"),
        13.. => (5, "SES"),
    };

    println!("Position: {}", positions[index]);
    println!("Level: {}", aps);
}

fn main() {
    let mut input = String::new();

    println!("Select your department:");
    println!("1. Office Worker");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let department: u8 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid department selection.");
            return;
        }
    };

    pick(department);
}
