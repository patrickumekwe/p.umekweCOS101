struct Employee {
    name : String,
    compnay : String,
    age :u32, // this is like a typescript interface
}

fn main() {
     let emp1 = Employee {
        compnay: String::from("PCU"),
        name: String::from("Patrick"),
        age: 16,
    };
    println!("Name = {} \n", emp1.name);
    println!("Company = {} \n", emp1.compnay);
    println!("Age =  {} ", emp1.age);
}
