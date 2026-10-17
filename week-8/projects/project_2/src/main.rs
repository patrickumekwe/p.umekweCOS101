// this one was hard small
use std::io;

fn main() {
    // Vector of tuples: (name and years of experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    println!("How many candidates?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let count: u32 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    for i in 1..=count {
        //to add candidates obv
        let mut name = String::new();
        println!("Candidate {} name:", i);
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");

        let mut years_input = String::new();
        println!("Years of programming experience:");
        io::stdin()
            .read_line(&mut years_input)
            .expect("Failed to read input");
        let years: u32 = match years_input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid years.");
                return;
            }
        };

        candidates.push((name.trim().to_string(), years));
    }

    if candidates.is_empty() {
        println!("No candidates entered.");
        return;
    }

    // if candidate before izs better keep if not i change it to the new one
    let mut best = &candidates[0];
    for candidate in &candidates {
        if candidate.1 > best.1 {
            best = candidate;
        }
    }

    println!("Most experienced: {} with {} years", best.0, best.1);
}
