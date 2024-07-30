use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DivisionResponse {
    pub id: i32,
    pub name: String
}