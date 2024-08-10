use serde::Deserialize;

use crate::deserialization::deserialize_non_empty_string;

#[derive(Deserialize)]
pub struct LocationCreate {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
}

#[derive(Deserialize)]
pub struct LocationUpdate {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
}
