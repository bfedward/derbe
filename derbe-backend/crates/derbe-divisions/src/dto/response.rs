use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DivisionResponse {
    pub id: i32,
    pub name: String,
}
