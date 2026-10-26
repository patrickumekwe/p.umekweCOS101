struct square {
    length: u32,
  
}


impl square {
    fn perimeter(&self) -> u32 {
       
        self.length * 4
    }
}

fn main() {
 
    let large = square {
        length: 10
    };
   
    println!(
        "length is {}  area of square is {}",
        large.length,
        large.perimeter()
    );
}
