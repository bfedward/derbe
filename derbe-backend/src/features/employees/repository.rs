use uuid::Uuid;

use crate::features::employees::dto::request::{EmployeeCreate, EmployeeUpdate};
use crate::features::employees::dto::response::{
    CreatedEmployeeResponse, EmployeeDetailsResponse, EmployeeResponse,
};

use crate::features::employees::model::EmployeeAccess;
use crate::{error::AppError, features::employees::model::EmploymentType, http::AppState};

pub async fn get_by_id(unid: Uuid, ctx: &AppState) -> Result<EmployeeResponse, AppError> {
    sqlx::query_as!(
        EmployeeResponse,
        r#"
            SELECT unid AS employee_unid, 
                    first_name, 
                    last_name, 
                    job_title, 
                    division_id, 
                    employment_type AS "employment_type!: EmploymentType", 
                    location_id, 
                    manager_unid, 
                    email
            FROM employees
            WHERE unid = $1
            "#,
        unid
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or_else(|| AppError::EntityUnidNotFound(unid, "Employee".to_owned()))
}

pub async fn get_access_by_id(unid: Uuid, ctx: &AppState) -> Result<EmployeeAccess, AppError> {
    sqlx::query_as!(
        EmployeeAccess,
        r#"
            SELECT unid,  
                    manager_unid            
            FROM employees
            WHERE unid = $1
            "#,
        unid
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or_else(|| AppError::EntityUnidNotFound(unid, "Employee".to_owned()))
}

pub async fn list(ctx: &AppState) -> Result<Vec<EmployeeResponse>, AppError> {
    Ok(sqlx::query_as!(
        EmployeeResponse,
        r#"
            SELECT unid AS employee_unid, 
                    first_name, 
                    last_name, 
                    job_title, 
                    division_id, 
                    employment_type AS "employment_type!: EmploymentType", 
                    location_id, 
                    manager_unid, 
                    email
            FROM employees
            "#
    )
    .fetch_all(&ctx.pool)
    .await?)
}

pub async fn create(
    dto: EmployeeCreate,
    ctx: &AppState,
) -> Result<CreatedEmployeeResponse, AppError> {
    let temporary_password = Uuid::new_v4().simple().to_string();
    let password_hash = pwhash::bcrypt::hash(&temporary_password).map_err(|e| {
        tracing::error!("Failed to hash temporary password: {e}");
        AppError::HashFailure
    })?;

    let employee = sqlx::query_as!(
        EmployeeResponse,
        r#"
            INSERT INTO employees (first_name, 
                                    last_name, 
                                    job_title, 
                                    division_id, 
                                    employment_type, 
                                    location_id, 
                                    manager_unid, 
                                    email,
                                    password)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING unid AS employee_unid, 
                        first_name, 
                        last_name, 
                        job_title, 
                        division_id, 
                        employment_type AS "employment_type!: EmploymentType", 
                        location_id, 
                        manager_unid, 
                        email
            "#,
        dto.first_name,
        dto.last_name,
        dto.job_title,
        dto.division_id,
        dto.employment_type as EmploymentType,
        dto.location_id,
        dto.manager_unid,
        dto.email,
        password_hash
    )
    .fetch_one(&ctx.pool)
    .await?;

    Ok(CreatedEmployeeResponse {
        employee,
        temporary_password,
    })
}

pub async fn update(
    unid: Uuid,
    dto: EmployeeUpdate,
    ctx: &AppState,
) -> Result<EmployeeResponse, AppError> {
    sqlx::query_as!(
        EmployeeResponse,
        r#"
            UPDATE employees
            SET first_name = $1,
                last_name = $2,
                job_title = $3,
                division_id = $4,
                employment_type = $5,
                location_id = $6,
                manager_unid = $7,
                email = $8
            WHERE unid = $9
            RETURNING unid AS employee_unid, 
                first_name, 
                last_name, 
                job_title, 
                division_id, 
                employment_type AS "employment_type!: EmploymentType", 
                location_id, 
                manager_unid, 
                email
            "#,
        dto.first_name,
        dto.last_name,
        dto.job_title,
        dto.division_id,
        dto.employment_type as EmploymentType,
        dto.location_id,
        dto.manager_unid,
        dto.email,
        unid
    )
    .fetch_optional(&ctx.pool)
    .await?
    .ok_or_else(|| AppError::EntityUnidNotFound(unid, "Employee".to_owned()))
}

pub async fn delete(unid: Uuid, ctx: &AppState) -> Result<(), AppError> {
    let _ = sqlx::query_scalar!("DELETE FROM employees WHERE unid = $1 RETURNING unid", unid)
        .fetch_optional(&ctx.pool)
        .await?;

    Ok(())
}
pub async fn ensure_exists(unid: Uuid, ctx: &AppState) -> Result<(), AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM employees WHERE unid = $1)",
        unid
    )
    .fetch_one(&ctx.pool)
    .await?
    .unwrap_or(false);

    if exists {
        Ok(())
    } else {
        Err(AppError::EntityUnidNotFound(unid, "Employee".to_owned()))
    }
}

pub async fn ensure_not_referenced(unid: Uuid, ctx: &AppState) -> Result<(), AppError> {
    let count = sqlx::query_scalar!(
        "
            SELECT ((SELECT COUNT(*) FROM employees WHERE manager_unid = $1) +
            (SELECT COUNT(*) FROM leave_requests WHERE employee_unid = $1)) AS count
            ",
        unid
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
                e.unid AS "employee_unid",
                CONCAT(e.first_name, ' ', e.last_name) AS "name!: std::string::String",
                e.job_title,
                d.name AS "division!: std::string::String",
                e.employment_type AS "employment_type!: std::string::String",
                l.name AS "location!: std::string::String",
                CONCAT(m.first_name, ' ', m.last_name) AS "manager!: std::option::Option<std::string::String>",
                e.email AS "email!: std::string::String"
            FROM
                employees e
            JOIN
                divisions d ON e.division_id = d.id
            JOIN
                locations l ON e.location_id = l.id
            LEFT JOIN
                employees m ON e.manager_unid = m.unid
            ORDER BY e.last_name, e.first_name
            "#
        )
        .fetch_all(&ctx.pool)
        .await?)
}
