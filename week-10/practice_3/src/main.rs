fn main(){
let v = vec![12, 40, 16, 80];

let v2 = v;
let v2_return = display(v2);
println!("In main {:?}",v);
}

fn display(v:Vec<i32>)->Vec<i32> {
// returning same vector
println!("Inside Display {:?}",v);
return v;
}
