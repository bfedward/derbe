use std::str::FromStr;

use serde::Deserialize;
use ts_rs::TS;
use uuid::Uuid;

use crate::{enums::EmploymentType, error::AppError};

use validator::Validate;

#[derive(Validate, Clone, Deserialize, Default, TS)]
#[serde(try_from = "EmployeeCreateRaw")]
#[ts(export)]
pub struct EmployeeCreate {
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(length(min = 1))]
    pub job_title: String,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    #[ts(type = "string")]
    pub manager_unid: Option<Uuid>,
    pub email: String,
}

#[derive(Deserialize)]
struct EmployeeCreateRaw {
    first_name: String,
    last_name: String,
    job_title: String,
    division_id: i32,
    employment_type: EmploymentType,
    location_id: i32,
    manager_unid: Option<String>,
    email: String,
}

impl TryFrom<EmployeeCreateRaw> for EmployeeCreate {
    type Error = AppError;

    fn try_from(raw: EmployeeCreateRaw) -> Result<Self, Self::Error> {
        let manager_unid: Option<Uuid> = raw
            .manager_unid
            .as_deref()
            .map(Uuid::from_str)
            .transpose()
            .map_err(|_| AppError::InvalidUuid(raw.manager_unid.clone().unwrap_or_default()))?;

        Ok(Self {
            first_name: raw.first_name,
            last_name: raw.last_name,
            job_title: raw.job_title,
            division_id: raw.division_id,
            employment_type: raw.employment_type,
            location_id: raw.location_id,
            manager_unid,
            email: raw.email,
        })
    }
}

#[derive(Validate, Deserialize, Clone, TS)]
#[serde(try_from = "EmployeeUpdateRaw")]
#[ts(export)]
pub struct EmployeeUpdate {
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(length(min = 1))]
    pub job_title: String,
    pub division_id: i32,
    pub employment_type: EmploymentType,
    pub location_id: i32,
    #[ts(type = "string")]
    pub manager_unid: Option<Uuid>,
    pub email: String,
}

#[derive(Deserialize)]
struct EmployeeUpdateRaw {
    first_name: String,
    last_name: String,
    job_title: String,
    division_id: i32,
    employment_type: EmploymentType,
    location_id: i32,
    manager_unid: Option<String>,
    email: String,
}

impl TryFrom<EmployeeUpdateRaw> for EmployeeUpdate {
    type Error = AppError;

    fn try_from(raw: EmployeeUpdateRaw) -> Result<Self, Self::Error> {
        let manager_unid: Option<Uuid> = raw
            .manager_unid
            .as_deref()
            .map(Uuid::from_str)
            .transpose()
            .map_err(|_| AppError::InvalidUuid(raw.manager_unid.clone().unwrap_or_default()))?;

        Ok(Self {
            first_name: raw.first_name,
            last_name: raw.last_name,
            job_title: raw.job_title,
            division_id: raw.division_id,
            employment_type: raw.employment_type,
            location_id: raw.location_id,
            manager_unid,
            email: raw.email,
        })
    }
}
