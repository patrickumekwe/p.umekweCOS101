use std::io;

fn ask_for_gender() -> String {
    let mut gender = String::new();

    println!("What is your gender? M/F");
    io::stdin()
        .read_line(&mut gender)
        .expect("Did not catch that");

    if gender.trim() != "M" && gender.trim() != "F" {
        println!("Please enter M or F");
        return ask_for_gender();
    }

    return gender.trim().to_string();
}
fn main(){
    let mut age = String::new();
    let mut exp = String::new();
    let gender = ask_for_gender();

    if gender =="F"
    {
        println!("I know you cant ask ladies their age,\n but can I know yours?");
        io::stdin().read_line(&mut age).expect("Did not catch that");
        let age:u8 = age.trim().parse().expect("try again");
        println!("Your age is {}", age);

    }else
    {
        println!("age");
        io::stdin().read_line(&mut age).expect("Did not catch that");
        let age:u8 = age.trim().parse().expect("try again");
        println!("Your age is {}", age);
    }

    println!("And do you have experience working in this field Y/N");
    io::stdin().read_line(&mut exp).expect("Did not catch that");
    let exp = exp.trim().to_uppercase();

    let years_exp: u8 = if exp == "Y" {
        println!("How many years of experience do you have?");
        let mut years = String::new();
        io::stdin().read_line(&mut years).expect("Did not catch that");
        years.trim().parse().expect("try again")
    } else {
        0
    };

    println!("Enter your salary:");
    let mut salary = String::new();
    io::stdin().read_line(&mut salary).expect("Did not catch that");
    let salary: f32 = salary.trim().parse().expect("salary isnt a number");

    let incentive = if years_exp >= 10 {
        0.20 * salary
    } else if years_exp >= 5 {
        0.10 * salary
    } else if years_exp >= 2 {
        0.05 * salary
    } else {
        0.02 * salary
    };

    println!("Your incentive is {}", incentive);

}
