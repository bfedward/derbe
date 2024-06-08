use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;
use crate::model::division::Division;

pub async fn get_divisions() -> Result<Vec<Division>, ApiError> {
    let req = Request::get(&format!("{}/divisions", BASE_URL))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Vec<Division>>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}

pub async fn get_division(id: &i32) -> Result<Division, ApiError> {
    let req = Request::get(&format!("{}/divisions/{}", BASE_URL, id))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Division>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}
