use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    noun: String,
    indexes: Vec<Index>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    name: String,
    fields: Vec<String>,
    options: Options,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    #[serde(default)]
    multi: bool,
}
pub fn from_file(filename: &str) -> Vec<Schema> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let schemas: Vec<Schema> = serde_json::from_reader(reader).unwrap();
    for part in &schemas {
        println!("schema {:?}", part);
    }
    schemas
}
