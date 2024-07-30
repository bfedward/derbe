use derbe_shared::dto::response::location::LocationResponse;

use crate::api::{
    api_errors::ApiError,
    locations::{get_location, get_locations},
};

use super::Selectable;

impl Selectable for LocationResponse {
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
