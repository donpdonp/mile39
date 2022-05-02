use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub type Schemas = HashMap<String, Vec<Index>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub noun: String,
    pub indexes: Vec<Index>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub name: String,
    pub fields: Vec<String>,
    pub options: Options,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    #[serde(default)]
    multi: bool,
}
pub fn from_file(filename: &str) -> Schemas {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let schemas: Schemas = serde_json::from_reader(reader).unwrap();
    for part in &schemas {
        println!("schema {:?}", part);
    }
    schemas
}
