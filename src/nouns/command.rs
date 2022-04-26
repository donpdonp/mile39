use serde::{Deserialize, Serialize};

use crate::nouns::location::Location;

#[derive(Serialize, Deserialize)]
pub enum Nouns {
    Location(Location),
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub verb: String,
    pub noun: Nouns,
}
