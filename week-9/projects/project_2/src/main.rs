use std::io::Write;
use std::fs::File;

fn main() {
    let lager = ["33 Export", "Desperados","Goldberg","Gulder","Heineken","Star"];
    let stout = ["Legend","Turbo king","Williams"];
    let non_alc = ["Maltina","Amstel Malt","Malta Gold","Fayrouz"]; // for ages 100 and below
    let mut list = String::from("Lager,Stout,Non-Alcholic,\n");
    let mut file = File::create("data.csv").expect("Failed create procees");
    
    for i in 0..lager.len(){
        list.push_str(&format!("{},", lager[i]));
        if(i<stout.len()){
           list.push_str(&format!("{},", stout[i]));
        }else{
            list.push_str(",");
        }
        if(i<non_alc.len()){
            list.push_str(&format!("{},", non_alc[i]));
        }else{
            list.push_str(",");
        }
        list.push_str("\n")

    }

    file.write_all(list.as_bytes()).expect("asdfghjk");

     
}
