use super::Selectable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Division {
    pub id: Option<i32>,
    pub name: String,
}

impl Selectable for Division {
    fn get_id(&self) -> i32 {
        self.id.unwrap_or(0)
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
