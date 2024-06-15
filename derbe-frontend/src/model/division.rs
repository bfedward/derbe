use crate::api::{
    api_errors::ApiError,
    divisions::{get_division, get_divisions},
};

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

    async fn get_entity(id: &i32) -> Result<Self, ApiError>
    where
        Self: Sized,
    {
        match get_division(id).await {
            Ok(res) => Ok(res),
            Err(error) => Err(error),
        }
    }

    async fn get_entities() -> Result<Vec<Self>, ApiError>
    where
        Self: Sized,
    {
        match get_divisions().await {
            Ok(res) => Ok(res),
            Err(error) => Err(error),
        }
    }
}
