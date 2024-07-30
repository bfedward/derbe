use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DivisionCreate {
    name: String
}

#[derive(Serialize, Deserialize)]
pub struct DivisionUpdate {
    id: i32,
    name: String
}