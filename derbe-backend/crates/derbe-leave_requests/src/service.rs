use derbe_core::{
    error::AppError,
    model::{EmployeeRole, LeaveStatus},
    session::AuthenticatedUser,
    state::AppState,
};
use derbe_email::{
    model::{EmailTemplate, LeaveRequestDecisionPayload, LeaveRequestSubmittedPayload},
    repository as email_repository,
};
use derbe_employees as employees;
use derbe_leave_types as leave_types;
use uuid::Uuid;

use crate::{
    dto::{
        request::{
            AllLeaveRequest, LeaveRequestCreate, LeaveRequestStatusUpdate, LeaveRequestUpdate,
            MyLeaveRequest,
        },
        response::{AllLeaveResponse, MyLeaveResponse, TeamLeaveResponse},
    },
    model::{LeaveRequest, LeaveRequestNames},
    repository,
};

pub async fn get_by_id(id: i32, ctx: &AppState) -> Result<LeaveRequest, AppError> {
    repository::get_by_id(id, ctx).await
}

pub async fn get_names(id: i32, ctx: &AppState) -> Result<LeaveRequestNames, AppError> {
    repository::get_names(id, ctx).await
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

pub async fn create(
    dto: LeaveRequestCreate,
    created_by: Uuid,
    ctx: &AppState,
) -> Result<LeaveRequest, AppError> {
    validate_create(&dto, ctx).await?;

    let mut tx = ctx.pool.begin().await?;
    let leave_request = repository::create(&mut tx, dto).await?;
    let details = repository::get_email_details(&mut tx, leave_request.id, created_by).await?;

    match (details.manager_unid, details.manager_email.as_deref()) {
        // A manager filing leave for their own report doesn't need an email about it.
        (Some(manager), _) if manager == created_by => {}
        (Some(_), Some(manager_email)) => {
            let payload = LeaveRequestSubmittedPayload {
                request_id: leave_request.id,
                employee_name: details.employee_name,
                leave_type: details.leave_type,
                date_from: leave_request.date_from,
                date_to: leave_request.date_to,
                total_duration: leave_request.total_duration,
                notes: leave_request.notes.clone(),
            };
            email_repository::enqueue(
                &mut tx,
                &format!("leave-request:{}:created", leave_request.id),
                manager_email,
                EmailTemplate::LeaveRequestSubmitted,
                &payload,
            )
            .await?;
        }
        _ => tracing::warn!(
            leave_request_id = leave_request.id,
            "employee has no active manager, no submission email queued"
        ),
    }

    tx.commit().await?;
    Ok(leave_request)
}

pub async fn validate_create(dto: &LeaveRequestCreate, ctx: &AppState) -> Result<(), AppError> {
    ensure_related_entities_exist(dto.employee_unid, dto.leave_type_id, ctx).await?;
    validate_new_request(dto)?;
    Ok(())
}

pub fn validate_new_request(dto: &LeaveRequestCreate) -> Result<(), AppError> {
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

async fn validate_status_update_rules(
    id: i32,
    dto: &LeaveRequestStatusUpdate,
    ctx: &AppState,
) -> Result<LeaveRequest, AppError> {
    let existing_entity = get_by_id(id, ctx).await?;
    let employee = employees::service::get_by_id(existing_entity.employee_unid, ctx).await?;

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
            employee.employee_unid,
            existing_entity.date_from,
            existing_entity.date_to,
            ctx,
        )
        .await?;
    }

    Ok(existing_entity)
}

pub async fn get_my_leave(
    employee_unid: Uuid,
    my_leave_request: MyLeaveRequest,
    ctx: &AppState,
) -> Result<Vec<MyLeaveResponse>, AppError> {
    repository::get_my_leave(employee_unid, my_leave_request, ctx).await
}

pub async fn get_team_leave(
    user: &AuthenticatedUser,
    ctx: &AppState,
) -> Result<Vec<TeamLeaveResponse>, AppError> {
    let manager_unid = match user.role {
        EmployeeRole::Admin => None,
        EmployeeRole::Manager => Some(user.unid),
        // The handler's permission check already rejects these. This is a second guard.
        EmployeeRole::RegularUser | EmployeeRole::Demo => return Err(AppError::Forbidden),
    };
    repository::get_team_leave(manager_unid, ctx).await
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

pub async fn update_leave_request_status(
    id: i32,
    request: LeaveRequestStatusUpdate,
    decided_by: Uuid,
    ctx: &AppState,
) -> Result<LeaveRequest, AppError> {
    let existing = validate_status_update_rules(id, &request, ctx).await?;
    let old_status = existing.leave_status;

    let mut tx = ctx.pool.begin().await?;
    let updated = repository::update_status(&mut tx, id, old_status, request).await?;

    let is_decision = matches!(
        updated.leave_status,
        LeaveStatus::Accepted | LeaveStatus::Rejected
    );
    if is_decision && updated.leave_status != old_status {
        let details = repository::get_email_details(&mut tx, id, decided_by).await?;
        let payload = LeaveRequestDecisionPayload {
            request_id: id,
            leave_type: details.leave_type,
            date_from: updated.date_from,
            date_to: updated.date_to,
            total_duration: updated.total_duration,
            decision: updated.leave_status,
            decided_by: details.actor_name,
        };
        email_repository::enqueue(
            &mut tx,
            &format!(
                "leave-request:{id}:{}",
                updated.leave_status.to_string().to_lowercase()
            ),
            &details.employee_email,
            EmailTemplate::LeaveRequestDecision,
            &payload,
        )
        .await?;
    }

    tx.commit().await?;
    Ok(updated)
}

pub async fn validate_update(
    id: i32,
    dto: &LeaveRequestUpdate,
    ctx: &AppState,
) -> Result<(), AppError> {
    get_by_id(id, ctx).await?;
    ensure_related_entities_exist(dto.employee_unid, dto.leave_type_id, ctx).await?;
    Ok(())
}
