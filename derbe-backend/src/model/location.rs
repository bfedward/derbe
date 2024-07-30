use derbe_shared::dto::response::location::LocationResponse;
use serde::{Deserialize, Serialize};

use super::{deserialization::deserialize_non_empty_string, DerbeModel};

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub id: i32,
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
}

impl DerbeModel for Location {}

impl From<LocationResponse> for Location {
    fn from(value: LocationResponse) -> Self {
        Self {
            id: value.id,
            name: value.name
        }
    }
}

impl From<Location> for LocationResponse {
    fn from(value: Location) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}