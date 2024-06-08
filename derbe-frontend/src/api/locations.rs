use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;
use crate::model::location::Location;

pub async fn get_locations() -> Result<Vec<Location>, ApiError> {
    let req = Request::get(&format!("{}/locations", BASE_URL))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Vec<Location>>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn get_location(id: &i32) -> Result<Location, ApiError> {
    let req = Request::get(&format!("{}/locations/{}", BASE_URL, id))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Location>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}
