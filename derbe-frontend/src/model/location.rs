use serde::{Deserialize, Serialize};

use crate::api::{
    api_errors::ApiError,
    locations::{get_location, get_locations},
};

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

    async fn get_entity(id: &i32) -> Result<Self, ApiError>
    where
        Self: Sized,
    {
        match get_location(id).await {
            Ok(res) => Ok(res),
            Err(error) => Err(error),
        }
    }

    async fn get_entities() -> Result<Vec<Self>, ApiError>
    where
        Self: Sized,
    {
        match get_locations().await {
            Ok(res) => Ok(res),
            Err(error) => Err(error),
        }
    }
}
