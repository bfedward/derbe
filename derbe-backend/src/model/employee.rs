use derbe_shared::{dto::response::employee::EmployeeResponse, enums::EmploymentType};

pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    pub manager_id: Option<i32>,
    pub email: String,
    pub phone: Option<String>,
}

impl From<Employee> for EmployeeResponse {
    fn from(value: Employee) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            job_title: value.job_title,
            division_id: value.division_id,
            employment_type: value.employment_type,
            location_id: value.location_id,
            manager_id: value.manager_id,
            email: value.email,
            phone: value.phone,
        }
    }
}
