use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod command;
pub mod location;

#[derive(Serialize, Deserialize)]
pub enum Nouns {
    Location(location::Location),
}

pub fn name_value(value: &serde_json::Value) -> (String, &serde_json::Map<String, Value>) {
    let jobj = value.as_object().unwrap();
    let name = jobj.keys().next().unwrap().to_owned();
    let v = jobj.get(&name).unwrap().as_object().unwrap();
    (name, v)
}
