use serde::Deserialize;
use ts_rs::TS;
use validator::Validate;

#[derive(Validate, Deserialize, TS)]
#[ts(export)]
pub struct DivisionCreate {
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Validate, Deserialize, TS)]
#[ts(export)]
pub struct DivisionUpdate {
    #[validate(length(min = 1))]
    pub name: String,
}
