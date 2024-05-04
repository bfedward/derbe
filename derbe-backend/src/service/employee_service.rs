use crate::{model::employee::Employee, repository::employee_repository::EmployeeRepository};

use super::DerbeService;



pub struct EmployeeService {}

impl DerbeService<Employee, EmployeeRepository> for EmployeeService {}