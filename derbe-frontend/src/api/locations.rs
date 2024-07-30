use derbe_shared::dto::response::location::LocationResponse;
use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;

pub async fn get_locations() -> Result<Vec<LocationResponse>, ApiError> {
    let req = Request::get(&format!("{}/locations", BASE_URL))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Vec<LocationResponse>>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn get_location(id: &i32) -> Result<LocationResponse, ApiError> {
    let req = Request::get(&format!("{}/locations/{}", BASE_URL, id))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<LocationResponse>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}
