use std::fs::remove_file;

fn main() {
   remove_file("data.txt").unwrap();
}
