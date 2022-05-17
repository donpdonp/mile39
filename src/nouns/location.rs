use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Location {
    pub id: String,
    pub date: String,
    pub user_id: String,
    pub lat: f32,
    pub lng: f32,
}
