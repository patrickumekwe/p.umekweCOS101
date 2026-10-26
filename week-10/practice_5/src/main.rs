
fn main() {
  
    let x = vec![100, 200, 300];
    borrow_vector(&x); // i am  passing the ref of ...


    println!("Printing the value from main() x[0]={}", x[0]);
    println!("*****************************");
}

fn borrow_vector(z: &Vec<i32>) {
    println!("*****************************");
    println!("Inside print_vector function {:?} \n", z);
    println!("-----------------------------");
}
