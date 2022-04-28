use serde::{Deserialize, Serialize};

use crate::nouns::*;

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub verb: String,
    pub noun: Nouns,
}
