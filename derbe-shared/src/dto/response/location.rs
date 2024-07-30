use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LocationResponse {
    pub id: i32,
    pub name: String
}