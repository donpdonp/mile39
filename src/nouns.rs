use serde::{Deserialize, Serialize};

pub mod command;
pub mod location;

#[derive(Serialize, Deserialize)]
pub enum Nouns {
    Location(location::Location),
}
