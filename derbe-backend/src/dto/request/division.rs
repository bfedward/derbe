use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::deserialization::deserialize_non_empty_string;

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DivisionCreate {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DivisionUpdate {
    #[serde(deserialize_with = "deserialize_non_empty_string")]
    pub name: String,
}
