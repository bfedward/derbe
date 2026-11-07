use serde::Deserialize;
use ts_rs::TS;

use crate::deserialization::deserialize_non_empty_string;

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct LocationCreate {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct LocationUpdate {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
}
