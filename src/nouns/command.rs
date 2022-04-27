use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub verb: String,
    pub noun: serde_json::Value,
}
