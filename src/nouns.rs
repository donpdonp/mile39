use serde::{Deserialize, Serialize};

pub mod location;
pub mod command;

#[derive(Serialize, Deserialize)]
pub enum Nouns {
    Location(location::Location)
}

pub fn to_string(noun: &Nouns) -> String {
    match noun {
        Location => "Location".to_string()
    }
}
