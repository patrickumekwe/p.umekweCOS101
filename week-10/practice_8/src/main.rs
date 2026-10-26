//declare
struct Employee {
    ceo: String,
    company: String,
    age: u32,
}

fn main() {
    //init
    let emp1 = Employee {
        company: String::from("PCU"),
        ceo: String::from("Patrick Umekwe"),
        age: 16,
    };
    let emp2 = Employee {
        company: String::from("Arravo"),
        ceo: String::from("Zion Adegboye"),
        age: 17,
    };
    
    display(emp1);
    display(emp2);
}


fn display(emp: Employee) {
    println!(
        "Name is :{} company is {} age is {}", emp.ceo, emp.company, emp.age
    );
}
