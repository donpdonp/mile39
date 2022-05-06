use serde::{Deserialize, Serialize};

use crate::nouns::*;

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub verb: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noun: Option<Nouns>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noun: Option<Nouns>,
}
