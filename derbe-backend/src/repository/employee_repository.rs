use crate::{error::AppError, http::AppContext, model::employee::Employee};

pub async fn get_employee_by_id(id: i32, ctx: &AppContext) -> Result<Employee, AppError> {
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