use std::fs::File;
use std::io::Write;

fn main() {
    let commisioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    let ministries = [
        "Internal Affairs",
        "justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];
    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut list = String::from("Commisioner,Ministry,Zone\n");

    let mut merged: Vec<String> = Vec::new();
    for i in 0..commisioners.len() {
        merged.push(format!("{},{},{}", commisioners[i], ministries[i], zones[i]));
    }

    for line in &merged {
        list.push_str(line);
        list.push_str("\n");
    }

    let mut file = File::create("data.csv").expect("Failed to create");
    file.write_all(list.as_bytes()).expect("Failed to write ");

   
}
