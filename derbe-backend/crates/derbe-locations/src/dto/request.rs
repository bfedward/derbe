use serde::Deserialize;
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Validate, TS)]
#[ts(export)]
pub struct LocationCreate {
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Deserialize, Validate, TS)]
#[ts(export)]
pub struct LocationUpdate {
    #[validate(length(min = 1))]
    pub name: String,
}
