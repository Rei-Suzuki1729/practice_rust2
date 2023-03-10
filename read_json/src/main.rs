use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Windriver{
    id: i32,
    product: String,
}

fn main() {
    let content = fs::read_to_string("windriver.json")
        .expect("Failed to load JSON");
    let deserialized: Vec<Windriver> = serde_json::from_str(&content).unwrap();
    println!("deserialized = {:?}", deserialized);
}
