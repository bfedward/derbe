use serde::{Deserialize, Serialize};

use super::Selectable;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Location {
    pub id: Option<i32>,
    pub name: String,
}

impl Selectable for Location {
    fn get_id(&self) -> i32 {
        self.id.unwrap_or(0)
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
