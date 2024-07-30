use derbe_shared::dto::response::division::DivisionResponse;
use serde::{Deserialize, Serialize};

use super::{deserialization::deserialize_non_empty_string, DerbeModel};

#[derive(Serialize, Deserialize)]
pub struct Division {
    pub id: i32,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
}

impl DerbeModel for Division {}

impl From<DivisionResponse> for Division {
    fn from(value: DivisionResponse) -> Self {
        Self {
            id: value.id,
            name: value.name
        }
    }
}

impl From<Division> for DivisionResponse {
    fn from(value: Division) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}