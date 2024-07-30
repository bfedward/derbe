use derbe_shared::dto::response::division::DivisionResponse;
use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;

pub async fn get_divisions() -> Result<Vec<DivisionResponse>, ApiError> {
    let req = Request::get(&format!("{}/divisions", BASE_URL))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Vec<DivisionResponse>>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn get_division(id: &i32) -> Result<DivisionResponse, ApiError> {
    let req = Request::get(&format!("{}/divisions/{}", BASE_URL, id))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<DivisionResponse>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}
