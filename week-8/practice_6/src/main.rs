fn main() {
    println!("Adding vectors"); //eigenverctor (math joke)

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let x = vec![10, 20, 30, 40, 50, 60, 70, 80];
    let mut sum = vec![0; v.len()];

    for index in 0..v.len() {
        sum[index] = v[index] + x[index];
    }

    println!("Vector sum: {:?}", sum);
}
