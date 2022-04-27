use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Location {
    lat: f32,
    lng: f32,
}
