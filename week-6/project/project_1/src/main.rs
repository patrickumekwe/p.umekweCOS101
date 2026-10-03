use std::io;
fn get_price(code: &str) -> i32 {
    let price = match code {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => 0,
    };
    return price;
}

fn get_item(code: &str) -> &str {
    match code {
        "P" => "Poundo Yam / Edinkaiko Soup",
        "F" => "Fried Rice & Chicken",
        "A" => "Amala & Ewedu Soup",
        "E" => "Eba & Egusi Soup",
        "W" => "White Rice & Stew",
        _ => "Invalid Item",
    }
}

fn main() {
    let mut order = String::new();
    let mut orders: Vec<String> = vec![];
    let mut total = 0.0;

    //iDisplay I used ai for this
    println!("==============================================");
    println!("              RESTAURANT MENU");
    println!("==============================================");
    println!();
    println!("  Code   Food                              Price");
    println!("  ----------------------------------------------");
    println!("   P     Poundo Yam / Edinkaiko Soup       N3,200");
    println!("   F     Fried Rice & Chicken              N3,000");
    println!("   A     Amala & Ewedu Soup                N2,500");
    println!("   E     Eba & Egusi Soup                  N2,000");
    println!("   W     White Rice & Stew                 N2,500");
    println!("==============================================");
    //AI stops here
    println!("\n what would you like to eat separate by commas e.g p,f,a");
    //Grab orders
    io::stdin().read_line(&mut order).expect("Try again");
    order = order.trim().to_string();
    let temps = order.split(",");
    for temp in temps {
        let temp = temp.to_uppercase().trim().to_string();
        orders.push(get_item(&temp).to_string());
        total += get_price(&temp) as f64;
    }

    //print statement
    //ai for this too
    println!();
    println!("==============================================");
    println!("                 YOUR ORDER");
    println!("==============================================");
    //break
    let mut i = 1;
    for o in orders {
        println!("   {}. {}", i, o);
        i += 1;
    }
    //this
    println!("==============================================");
    //break
    let total = if total > 10_000.0 {
        println!("   Discount: 5% applied");
        total * 0.95
    } else {
        println!("   No discount applied");
        total
    };
    println!("   Total: N{}", total);
    println!("==============================================");
}
