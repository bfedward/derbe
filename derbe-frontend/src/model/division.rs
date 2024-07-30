use crate::api::{
    api_errors::ApiError,
    divisions::{get_division, get_divisions},
};

use super::Selectable;
use derbe_shared::dto::response::division::DivisionResponse;

impl Selectable for DivisionResponse {
    fn get_id(&self) -> i32 {
        self.id
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
