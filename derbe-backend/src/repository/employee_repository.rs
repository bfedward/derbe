use crate::{
    dto::response::employee::EmployeeDetailsResponse, error::AppError, http::AppContext,
    model::employee::Employee,
};

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
        match sqlx::query_as!(
            Employee,
            r#"
            INSERT INTO employees (first_name, last_name, job_title, division_id, employment_type, 
                location_id, manager_id, email, phone)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, first_name, last_name, job_title, division_id, employment_type, 
                location_id, manager_id, email, phone
            "#,
            new_entity.first_name,
            new_entity.last_name,
            new_entity.job_title,
            new_entity.division_id,
            new_entity.employment_type,
            new_entity.location_id,
            new_entity.manager_id,
            new_entity.email,
            new_entity.phone
        )
        .fetch_one(&ctx.db)
        .await
        {
            Ok(employee) => Ok(employee),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn update_entity(
        id: i32,
        entity_update: Employee,
        ctx: &AppContext,
    ) -> Result<Employee, AppError> {
        match sqlx::query_as!(
            Employee,
            r#"
            UPDATE employees
            SET first_name = $1,
            last_name = $2,
            job_title = $3,
            division_id = $4,
            employment_type = $5,
            location_id = $6,
            manager_id = $7,
            email = $8,
            phone = $9
            WHERE id = $10
            RETURNING id, first_name, last_name, job_title, division_id, employment_type, 
                location_id, manager_id, email, phone
            "#,
            entity_update.first_name,
            entity_update.last_name,
            entity_update.job_title,
            entity_update.division_id,
            entity_update.employment_type,
            entity_update.location_id,
            entity_update.manager_id,
            entity_update.email,
            entity_update.phone,
            id
        )
        .fetch_optional(&ctx.db)
        .await
        {
            Ok(employee) => match employee {
                Some(employee) => Ok(employee),
                None => Err(AppError::EntityNotFound(id, "Employee".to_owned())),
            },
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn delete_entity(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        match sqlx::query!(
            r#"DELETE FROM employees
            WHERE id = $1
            "#,
            id
        )
        .execute(&ctx.db)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }

    async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        let exists =
            sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM employees WHERE id = $1)", id)
                .fetch_one(&ctx.db)
                .await
                .map_err(AppError::DatabaseQueryError)?;

        if exists.unwrap_or(false) {
            Ok(())
        } else {
            Err(AppError::EntityNotFound(id, "Employee".to_owned()))
        }
    }

    async fn check_if_entity_is_referenced(id: i32, ctx: &AppContext) -> Result<(), AppError> {
        let count = sqlx::query_scalar!(
            "
            SELECT ((SELECT COUNT(*) FROM employees WHERE manager_id = $1) +
            (SELECT COUNT(*) FROM leave_requests WHERE employee_id = $1)) AS count
            ",
            id
        )
        .fetch_one(&ctx.db)
        .await
        .map_err(AppError::DatabaseQueryError)?
        .unwrap_or(0);

        if count > 0 {
            Err(AppError::RelatedEntity("Employee".to_owned()))
        } else {
            Ok(())
        }
    }
}

impl EmployeeRepository {
    pub async fn get_employee_details_table(
        ctx: &AppContext,
    ) -> Result<Vec<EmployeeDetailsResponse>, AppError> {
        match sqlx::query_as!(
            EmployeeDetailsResponse,
            r#"
            SELECT
                e.id,
                CONCAT(e.first_name, ' ', e.last_name) AS "name!: std::string::String",
                e.job_title,
                d.name AS "division!: std::string::String",
                e.employment_type AS "employment_type!: std::string::String",
                l.name AS "location!: std::string::String",
                CONCAT(m.first_name, ' ', m.last_name) AS "manager!: std::option::Option<std::string::String>",
                e.email AS "email!: std::string::String",
                e.phone AS "phone!: std::option::Option<std::string::String>"
            FROM
                employees e
            JOIN
                divisions d ON e.division_id = d.id
            JOIN
                locations l ON e.location_id = l.id
            LEFT JOIN
                employees m ON e.manager_id = m.id
            ORDER BY e.last_name, e.first_name
            "#
        )
        .fetch_all(&ctx.db)
        .await
        {
            Ok(employees) => Ok(employees),
            Err(e) => Err(AppError::DatabaseQueryError(e)),
        }
    }
}
