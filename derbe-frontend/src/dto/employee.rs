use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmployeeDetailsResponse {
    pub id: i32,
    pub name: String,
    pub job_title: String,
    pub division: String,
    pub employment_type: String,
    pub location: String,
    pub manager: Option<String>,
    pub email: String,
    pub phone: Option<String>,
}
