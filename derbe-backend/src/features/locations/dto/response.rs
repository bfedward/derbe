use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LocationResponse {
    pub id: i32,
    pub name: String,
}
