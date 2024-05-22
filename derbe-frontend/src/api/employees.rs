use reqwasm::http::Request;

use crate::api::api_errors::{handle_errors, ApiError};
use crate::api::BASE_URL;
use crate::model::employee::Employee;

pub async fn get_employees() -> Result<Vec<Employee>, ApiError> {
    let req = Request::get(&format!("{}/employees", BASE_URL))
        .send()
        .await
        .unwrap();

    if req.ok() {
        Ok(req.json::<Vec<Employee>>().await.unwrap())
    } else {
        Err(handle_errors(req.status()))
    }
}
