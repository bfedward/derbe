use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Location {
    pub id: Option<i32>,
    pub name: String,
}
