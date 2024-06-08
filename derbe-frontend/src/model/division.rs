use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Division {
    pub id: Option<i32>,
    pub name: String,
}
