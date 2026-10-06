fn mutate_num_to_zero(param_num: &mut i32) {
    *param_num *= 0;
}

fn main() {
    let mut num = 5;
    mutate_num_to_zero(&mut num);
    println!("num is {}", num); // nevermind its gone
}
