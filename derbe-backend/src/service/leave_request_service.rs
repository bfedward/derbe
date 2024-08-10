use chrono::{DateTime, NaiveDate, Utc};

use derbe_shared::{
    dto::{
        request::leave_request::{
            AllLeaveRequest, LeaveRequestCreate, LeaveRequestStatusUpdate, LeaveRequestUpdate,
            MyLeaveRequest,
        },
        response::leave_request::{AllLeaveResponse, MyLeaveResponse},
    },
    enums::LeaveStatus,
};

use crate::{
    error::AppError, http::AppContext, model::leave_request::LeaveRequest,
    repository::leave_request_repository,
};

use super::{employee_service, leave_types_service};

pub async fn get_entity_by_id(id: i32, ctx: &AppContext) -> Result<LeaveRequest, AppError> {
    leave_request_repository::get_entity_by_id(id, ctx).await
}

pub async fn check_entity_exists(id: i32, ctx: &AppContext) -> Result<(), AppError> {
    leave_request_repository::check_entity_exists(id, ctx).await
}

pub async fn check_related_entities_exist(
    employee_id: i32,
    leave_type_id: i32,
    ctx: &AppContext,
) -> Result<(), AppError> {
    employee_service::check_entity_exists(employee_id, ctx).await?;
    leave_types_service::check_entity_exists(leave_type_id, ctx).await?;
    Ok(())
}

pub async fn create_entity(
    dto: LeaveRequestCreate,
    ctx: &AppContext,
) -> Result<LeaveRequest, AppError> {
    pre_create(&dto, ctx).await?;
    leave_request_repository::create_entity(dto, ctx).await
}

pub async fn pre_create(dto: &LeaveRequestCreate, ctx: &AppContext) -> Result<(), AppError> {
    check_related_entities_exist(dto.employee_id, dto.leave_type_id, ctx).await?;
    pre_create_validation(dto, ctx).await?;
    Ok(())
}

pub async fn pre_create_validation(
    dto: &LeaveRequestCreate,
    _ctx: &AppContext,
) -> Result<(), AppError> {
    pre_create_and_pre_update_validation(
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

pub async fn pre_update_validation(
    id: i32,
    dto: &LeaveRequestUpdate,
    ctx: &AppContext,
) -> Result<(), AppError> {
    pre_create_and_pre_update_validation(
        &dto.values.date_from,
        &dto.values.date_to,
        &dto.date_requested,
    )?;
    let existing_entity = get_entity_by_id(id, ctx).await?;

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
        leave_request_repository::check_for_accepted_leave_for_employee_and_date_range(
            dto.employee_id,
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

fn pre_create_and_pre_update_validation(
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
    ctx: &AppContext,
) -> Result<Vec<MyLeaveResponse>, AppError> {
    leave_request_repository::get_my_leave(my_leave_request, ctx).await
}

pub async fn get_all_leave_table_data(
    all_leave_request: AllLeaveRequest,
    ctx: &AppContext,
) -> Result<Vec<AllLeaveResponse>, AppError> {
    leave_request_repository::get_all_leave_table_data(all_leave_request, ctx).await
}

pub async fn update_entity(
    id: i32,
    dto: LeaveRequestUpdate,
    ctx: &AppContext,
) -> Result<LeaveRequest, AppError> {
    pre_update(id, &dto, ctx).await?;
    leave_request_repository::update_entity(id, dto, ctx).await
}

pub async fn pre_update(
    id: i32,
    dto: &LeaveRequestUpdate,
    ctx: &AppContext,
) -> Result<(), AppError> {
    check_related_entities_exist(dto.employee_id, dto.leave_type_id, ctx).await?;
    pre_update_validation(id, dto, ctx).await?;
    Ok(())
}

pub async fn update_leave_request_status(
    id: i32,
    request: LeaveRequestStatusUpdate,
    ctx: &AppContext,
) -> Result<LeaveRequest, AppError> {
    let mut updated_leave_request = get_entity_by_id(id, ctx).await?;
    updated_leave_request.leave_status = request.leave_status;
    updated_leave_request.notes = request.notes;

    if request.leave_status == LeaveStatus::Accepted
        || request.leave_status == LeaveStatus::Rejected
    {
        updated_leave_request.date_finalised = Some(Utc::now());
    }

    updated_leave_request = update_entity(id, updated_leave_request.into(), ctx).await?;

    Ok(updated_leave_request)
}
