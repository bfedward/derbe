use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LocationCreate {
    name: String
}

#[derive(Deserialize)]
pub struct LocationUpdate {
    id: i32,
    name: String
}