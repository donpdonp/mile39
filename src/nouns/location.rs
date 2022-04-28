use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    pub lat: f32,
    pub lng: f32,
}
