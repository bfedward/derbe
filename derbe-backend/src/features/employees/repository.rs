use crate::enums::EmploymentType;
use crate::features::employees::dto::request::{EmployeeCreate, EmployeeUpdate};
use crate::features::employees::dto::response::EmployeeDetailsResponse;

use crate::{error::AppError, features::employees::model::Employee, http::AppState};

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<Employee, AppError> {
    sqlx::query_as!(
        Employee,
        r#"
            SELECT id, 
                    first_name, 
                    last_name, 
                    job_title, 
                    division_id, 
                    employment_type AS "employment_type!: EmploymentType", 
                    location_id, 
                    manager_id, 
                    email,
                    phone
            FROM employees
            WHERE id = $1
            "#,
        id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or_else(|| AppError::EntityNotFound(id, "Employee".to_owned()))
}

pub async fn list(ctx: &AppState) -> Result<Vec<Employee>, AppError> {
    Ok(sqlx::query_as!(
        Employee,
        r#"
            SELECT id, 
                    first_name, 
                    last_name, 
                    job_title, 
                    division_id, 
                    employment_type AS "employment_type!: EmploymentType", 
                    location_id, 
                    manager_id, 
                    email, 
                    phone
            FROM employees
            "#
    )
    .fetch_all(&ctx.pool)
    .await?)
}

pub async fn create(create_dto: EmployeeCreate, ctx: &AppState) -> Result<Employee, AppError> {
    Ok(sqlx::query_as!(
        Employee,
        r#"
            INSERT INTO employees (first_name, 
                                    last_name, 
                                    job_title, 
                                    division_id, 
                                    employment_type, 
                                    location_id, 
                                    manager_id, 
                                    email, 
                                    phone)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, 
                        first_name, 
                        last_name, 
                        job_title, 
                        division_id, 
                        employment_type AS "employment_type!: EmploymentType", 
                        location_id, 
                        manager_id, 
                        email, 
                        phone
            "#,
        create_dto.first_name,
        create_dto.last_name,
        create_dto.job_title,
        create_dto.division_id,
        create_dto.employment_type as EmploymentType,
        create_dto.location_id,
        create_dto.manager_id,
        create_dto.email,
        create_dto.phone
    )
    .fetch_one(&ctx.pool)
    .await?)
}

pub async fn update(id: i32, dto: EmployeeUpdate, ctx: &AppState) -> Result<Employee, AppError> {
    sqlx::query_as!(
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
            RETURNING id, 
                first_name, 
                last_name, 
                job_title, 
                division_id, 
                employment_type AS "employment_type!: EmploymentType", 
                location_id, 
                manager_id, 
                email, 
                phone
            "#,
        dto.first_name,
        dto.last_name,
        dto.job_title,
        dto.division_id,
        dto.employment_type as EmploymentType,
        dto.location_id,
        dto.manager_id,
        dto.email,
        dto.phone,
        id
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or_else(|| AppError::EntityNotFound(id, "Employee".to_owned()))
}

pub async fn delete(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let _ = sqlx::query_scalar!("DELETE FROM employees WHERE id = $1 RETURNING id", id)
        .fetch_optional(&ctx.pool)
        .await?;

    Ok(())
}
pub async fn ensure_exists(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM employees WHERE id = $1)", id)
        .fetch_one(&ctx.pool)
        .await?
        .unwrap_or(false);

    if exists {
        Ok(())
    } else {
        Err(AppError::EntityNotFound(id, "Employee".to_owned()))
    }
}

pub async fn ensure_not_referenced(id: i32, ctx: &AppState) -> Result<(), AppError> {
    let count = sqlx::query_scalar!(
        "
            SELECT ((SELECT COUNT(*) FROM employees WHERE manager_id = $1) +
            (SELECT COUNT(*) FROM leave_requests WHERE employee_id = $1)) AS count
            ",
        id
    )
    .fetch_one(&ctx.pool)
    .await?
    .unwrap_or(0);

    if count > 0 {
        Err(AppError::RelatedEntity("Employee".to_owned()))
    } else {
        Ok(())
    }
}

pub async fn get_employee_details_table(
    ctx: &AppState,
) -> Result<Vec<EmployeeDetailsResponse>, AppError> {
    Ok(sqlx::query_as!(
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
        .fetch_all(&ctx.pool)
        .await?)
}
