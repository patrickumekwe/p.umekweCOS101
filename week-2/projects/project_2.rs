fn main() {
    let toshiba_qty = 3;
    let toshiba_amount= 450000.0;
    let mac_qty= 1;
    let mac_amount= 1500000.0;
    let hp_qty= 3;
    let hp_amount= 750000.0;
    let dell_qty= 3;
    let dell_amount= 2850000.0;
    let acer_qty = 1;
    let acer_amount= 250000.0;
    let toshiba_total = toshiba_qty as f64 * toshiba_amount;
    let mac_total = mac_qty as f64 * mac_amount;
    let hp_total= hp_qty as f64 * hp_amount;
    let dell_total = dell_qty as f64 * dell_amount;
    let acer_total = acer_qty as f64 * acer_amount;

    let sum= toshiba_total + mac_total + hp_total+ dell_total + acer_total;

    let total_quantity= toshiba_qty+ mac_qty + hp_qty + dell_qty + acer_qty;

    let average= sum / total_quantity as f64;

    println!("You sold ₦{:.2} worth of products",sum);
    println!("You averaged :₦{:.2}", average);
}