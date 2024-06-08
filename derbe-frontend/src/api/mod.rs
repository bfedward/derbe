pub mod api_errors;
pub mod divisions;
pub mod employees;
pub mod leave_requests;
pub mod leave_types;
pub mod locations;

const BASE_URL: &str = include_str!("api_base_uri.txt");
