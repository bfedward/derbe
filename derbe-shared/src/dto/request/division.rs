use serde::{Deserialize, Serialize};

use crate::deserialization::deserialize_non_empty_string;

#[derive(Serialize, Deserialize)]
pub struct DivisionCreate {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DivisionUpdate {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
}
