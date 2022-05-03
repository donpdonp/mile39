use serde::{Deserialize, Serialize};

pub mod command;
pub mod location;

#[derive(Serialize, Deserialize)]
pub enum Nouns {
    Location(location::Location),
}

pub fn to_string(noun: &Nouns) -> String {
    match noun {
        Nouns::Location(_) => "Location".to_string(),
    }
}
