
struct Laptop {
    brand: String,
    price: u64, 
}

impl Laptop {
    fn cost(&self, qty: u64) -> u64 {
        self.price * qty
    }
}

fn naira(n: u64) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    let mut count = 0;

    for ch in digits.chars().rev() {
        if count == 3 {
            out.push(',');
            count = 0;
        }
        out.push(ch);
        count += 1;
    }

    let out: String = out.chars().rev().collect();
    format!("N{}", out)
}

fn main() {
    let laptops = vec![
        Laptop { brand: String::from("HP"), price: 650_000 },
        Laptop { brand: String::from("IBM"), price: 755_000 },
        Laptop { brand: String::from("Toshiba"), price: 550_000 },
        Laptop { brand: String::from("Dell"), price: 850_000 },
    ];

    let qty_per_brand = 3;
    let mut total: u64 = 0;

    for laptop in &laptops {
        let cost = laptop.cost(qty_per_brand);
        println!(
            "{:<8} {} x {} = {}",
            laptop.brand,
            qty_per_brand,
            naira(laptop.price),
            naira(cost)
        );
        total += cost;
    }

    println!("Total cost for {} laptops = {}", qty_per_brand * laptops.len() as u64, naira(total));
}
