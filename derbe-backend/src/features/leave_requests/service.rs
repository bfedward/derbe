use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::{
    enums::LeaveStatus,
    features::leave_requests::dto::{
        request::{
            AllLeaveRequest, LeaveRequestCreate, LeaveRequestStatusUpdate, LeaveRequestUpdate,
            MyLeaveRequest,
        },
        response::{AllLeaveResponse, MyLeaveResponse},
    },
};

use crate::{
    error::AppError, features::leave_requests::model::LeaveRequest,
    features::leave_requests::repository, http::AppState,
};

use crate::features::{employees, leave_types};

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<LeaveRequest, AppError> {
    repository::get_by_id(id, ctx).await
}

pub async fn ensure_exists(id: i32, ctx: &AppState) -> Result<(), AppError> {
    repository::ensure_exists(id, ctx).await
}

pub async fn ensure_related_entities_exist(
    employee_unid: Uuid,
    leave_type_id: i32,
    ctx: &AppState,
) -> Result<(), AppError> {
    employees::service::ensure_exists(employee_unid, ctx).await?;
    leave_types::service::ensure_exists(leave_type_id, ctx).await?;
    Ok(())
}

pub async fn create(dto: LeaveRequestCreate, ctx: &AppState) -> Result<LeaveRequest, AppError> {
    validate_create(&dto, ctx).await?;
    repository::create(dto, ctx).await
}

pub async fn validate_create(dto: &LeaveRequestCreate, ctx: &AppState) -> Result<(), AppError> {
    let employee_unid = Uuid::from_str(&dto.employee_unid)
        .map_err(|_| AppError::InvalidUuid(dto.employee_unid.clone()))?;
    ensure_related_entities_exist(employee_unid, dto.leave_type_id, ctx).await?;
    validate_new_request(dto)?;
    Ok(())
}

pub fn validate_new_request(dto: &LeaveRequestCreate) -> Result<(), AppError> {
    validate_leave_request_dates(
        &dto.values.date_from,
        &dto.values.date_to,
        &dto.date_requested,
    )?;

    if dto.leave_status != LeaveStatus::Pending {
        return Err(AppError::ValidationFailed(
            "New leave requests must have Pending status".to_owned(),
        ));
    }

    // is entity.total_duration correct given date_from, date_to, first/last day half etc?

    if dto.date_finalised.is_some() {
        return Err(AppError::ValidationFailed(
            "New leave requests cannot have a finalised date".to_owned(),
        ));
    };

    Ok(())
}

pub async fn validate_update_rules(
    id: i32,
    dto: &LeaveRequestUpdate,
    ctx: &AppState,
) -> Result<(), AppError> {
    let employee_unid = Uuid::from_str(&dto.employee_unid)
        .map_err(|_| AppError::InvalidUuid(dto.employee_unid.clone()))?;

    validate_leave_request_dates(
        &dto.values.date_from,
        &dto.values.date_to,
        &dto.date_requested,
    )?;
    let existing_entity = get_by_id(id, ctx).await?;

    let leave_status_state_change_ok: bool = matches!(
        (&existing_entity.leave_status, &dto.leave_status),
        (LeaveStatus::Pending, _)
            | (LeaveStatus::MoreInfoReq, LeaveStatus::Pending)
            | (LeaveStatus::MoreInfoReq, LeaveStatus::MoreInfoReq)
            | (LeaveStatus::MoreInfoReq, LeaveStatus::Cancelled)
            | (LeaveStatus::Cancelled, LeaveStatus::Cancelled)
            | (LeaveStatus::Rejected, LeaveStatus::Rejected)
            | (LeaveStatus::Accepted, LeaveStatus::Accepted)
    );

    if !leave_status_state_change_ok {
        return Err(AppError::ValidationFailed(format!(
            "Cannot move from {} to {}",
            existing_entity.leave_status, dto.leave_status
        )));
    }

    if dto.leave_status == LeaveStatus::Accepted
        && existing_entity.leave_status != LeaveStatus::Accepted
    {
        repository::ensure_no_accepted_leave_conflict(
            employee_unid,
            dto.values.date_from,
            dto.values.date_to,
            ctx,
        )
        .await?;
    }

    if (dto.leave_status == LeaveStatus::Accepted || dto.leave_status == LeaveStatus::Rejected)
        && dto.date_finalised.is_none()
    {
        return Err(AppError::ValidationFailed(
            "Accepted and Rejected leave requests must have a finalised date".to_string(),
        ));
    }

    Ok(())
}

fn validate_leave_request_dates(
    date_from: &NaiveDate,
    date_to: &NaiveDate,
    date_requested: &DateTime<Utc>,
) -> Result<(), AppError> {
    if date_to < date_from {
        return Err(AppError::ValidationFailed(
            "Date To cannot be before Date From".to_owned(),
        ));
    }

    if *date_requested > Utc::now() {
        return Err(AppError::ValidationFailed(
            "Date Requested cannot be in the future".to_owned(),
        ));
    }

    Ok(())
}

pub async fn get_my_leave(
    my_leave_request: MyLeaveRequest,
    ctx: &AppState,
) -> Result<Vec<MyLeaveResponse>, AppError> {
    repository::get_my_leave(my_leave_request, ctx).await
}

pub async fn get_all_leave(
    all_leave_request: AllLeaveRequest,
    ctx: &AppState,
) -> Result<Vec<AllLeaveResponse>, AppError> {
    repository::get_all_leave(all_leave_request, ctx).await
}

pub async fn update(
    id: i32,
    dto: LeaveRequestUpdate,
    ctx: &AppState,
) -> Result<LeaveRequest, AppError> {
    validate_update(id, &dto, ctx).await?;
    repository::update(id, dto, ctx).await
}

pub async fn validate_update(
    id: i32,
    dto: &LeaveRequestUpdate,
    ctx: &AppState,
) -> Result<(), AppError> {
    let employee_unid = Uuid::from_str(&dto.employee_unid)
        .map_err(|_| AppError::InvalidUuid(dto.employee_unid.clone()))?;
    ensure_related_entities_exist(employee_unid, dto.leave_type_id, ctx).await?;
    validate_update_rules(id, dto, ctx).await?;
    Ok(())
}

pub async fn update_leave_request_status(
    id: i32,
    request: LeaveRequestStatusUpdate,
    ctx: &AppState,
) -> Result<LeaveRequest, AppError> {
    let mut updated_leave_request = get_by_id(id, ctx).await?;
    updated_leave_request.leave_status = request.leave_status;
    updated_leave_request.notes = request.notes;

    if request.leave_status == LeaveStatus::Accepted
        || request.leave_status == LeaveStatus::Rejected
    {
        updated_leave_request.date_finalised = Some(Utc::now());
    }

    updated_leave_request = update(id, updated_leave_request.into(), ctx).await?;

    Ok(updated_leave_request)
}
