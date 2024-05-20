use serde::{Deserialize, Serialize};

use super::{deserialization::deserialize_non_empty_string, DerbeModel};

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub id: Option<i32>,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
}

impl DerbeModel for Location {}
