use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
    pub division_id: i32,
    pub employment_type: String,
    pub location_id: i32,
    pub manager_id: Option<i32>,
    pub email: String,
    pub phone: Option<String>,
}
