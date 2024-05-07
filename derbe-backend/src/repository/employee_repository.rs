use crate::{error::AppError, http::AppContext, model::employee::Employee};

use super::DerbeRepository;

pub struct EmployeeRepository {}

impl DerbeRepository<Employee> for EmployeeRepository {
    async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<Employee, AppError> {
        match sqlx::query_as!(
            Employee,
            r#"
            SELECT id, first_name, last_name, job_title, division_id, employment_type, 
                location_id, manager_id, email, phone
            FROM employees
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&ctx.db)
        .await
        {
            Ok(Some(employee)) => Ok(employee),
            Ok(None) => Err(AppError::EntityNotFound(id, "Employee".to_owned())),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn get_entities(ctx: &AppContext) -> Result<Vec<Employee>, AppError> {
        match sqlx::query_as!(
            Employee,
            r#"
            SELECT id, first_name, last_name, job_title, division_id, employment_type, 
                location_id, manager_id, email, phone
            FROM employees
            "#
        )
        .fetch_all(&ctx.db)
        .await
        {
            Ok(employees) => Ok(employees),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn create_entity(new_entity: Employee, ctx: &AppContext) -> Result<Employee, AppError> {
        todo!()
    }

    async fn update_entity(
        entity_update: Employee,
        ctx: &AppContext,
    ) -> Result<Employee, AppError> {
        todo!()
    }

    async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        todo!()
    }
}
