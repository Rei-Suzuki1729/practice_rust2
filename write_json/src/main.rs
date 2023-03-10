use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Windriver{
    id: i32,
    product: String,
}

fn main() {
    let vx = Windriver { id: 100, product: String::from("VxWorks") };
    let linux = Windriver { id: 200, product: String::from("Wind River Linux") };
    let simics = Windriver { id: 300, product: String::from("Simics") };
    let diab =  Windriver { id: 400, product: String::from("Diab") };
    let helix = Windriver { id: 500, product: String::from("Helix Virtualization platform ") };

    let mut windrivers: Vec<Windriver> = Vec::new();
    windrivers.push(vx);
    windrivers.push(linux);
    windrivers.push(simics);
    windrivers.push(diab);
    windrivers.push(helix);	


    let result = write_file(windrivers);
    match result {
        Ok(..) => { println!("Write Finished") }
        Err(err) => { println!("Failed to Write: {}", err) }
    }
}

fn write_file(windrivers: Vec<Windriver>) -> std::io::Result<()> {
    // serialized
    let serialized: String = serde_json::to_string(&windrivers).unwrap();

    // write
    let mut file = File::create("windriver.json")?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
