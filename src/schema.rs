use std::fs::File;
use std::io::BufReader;
pub struct Schema {}

pub fn new(filename: &str) -> Schema {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u : serde_json::Value = serde_json::from_reader(reader).unwrap();
    println!("schema {:?}", u);
    Schema {}
}
