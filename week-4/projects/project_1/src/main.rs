use std::io;

fn main() {
    println!("Quadratic Calculator");
    let mut a = String::new();
    let mut b= String::new();
    let mut c = String::new();

    io::stdin().read_line(&mut a).expect("didnt catch that 😔");
    let a:f32 = a.trim().parse().expect("a isnt an integer");
    
    io::stdin().read_line(&mut b).expect("didnt catch that 😔");
    let b:f32 = b.trim().parse().expect("b isnt an integer");
    
    io::stdin().read_line(&mut c).expect("didnt catch that 😔");
    let c:f32 = c.trim().parse().expect("a isnt an integer");
    
    let det= b*b-4.0*a*c;
    if det >0.0
    {
        let x=(-b + det.sqrt() ) /(2.0*a);
        let xx=(-b - det.sqrt()) /(2.0*a);
        println!("Your first solution is {} and the second {}" , x,xx);
    } else if det == 0.0
    {
        let x=(-b + det.sqrt() ) /(2.0*a);
        println!("Your solution is lonely but it is {}" , x);
    }else
    {
          let x=(-b + det.sqrt() ) /(2.0*a);
           println!("You'll have to imagine them I'm afraid");
    }
    println!("Determinant \t{}",det);
}
