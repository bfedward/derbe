use chrono::Utc;

use derbe_shared::dto::{request::leave_request::AllLeaveRequest, response::leave_request::AllLeaveResponse};
use LeaveStatus::*;

use crate::{
    dto::{
        request::leave_request::{LeaveRequestStatusUpdate, MyLeaveRequest},
        response::leave_request::MyLeaveResponse,
    },
    error::AppError,
    http::AppContext,
    model::leave_request::{LeaveRequest, LeaveStatus},
    repository::leave_request_respository::LeaveRequestRepository,
};

use super::{
    employee_service::EmployeeService, leave_types_service::LeaveTypesService, DerbeService,
};

pub struct LeaveRequestService {}

impl DerbeService<LeaveRequest, LeaveRequestRepository> for LeaveRequestService {
    async fn check_related_entities_exist(
        entity: &LeaveRequest,
        ctx: &AppContext,
    ) -> Result<(), AppError> {
        EmployeeService::check_entity_exists(entity.employee_id, ctx).await?;
        LeaveTypesService::check_entity_exists(entity.leave_type_id, ctx).await?;
        Ok(())
    }

    async fn pre_create_validation(
        entity: &LeaveRequest,
        _ctx: &AppContext,
    ) -> Result<(), AppError> {
        Self::pre_create_and_pre_update_validation(entity)?;

        if entity.leave_status != LeaveStatus::Pending {
            return Err(AppError::ValidationFailed(
                "New leave requests must have Pending status".to_owned(),
            ));
        }

        // is entity.total_duration correct given date_from, date_to, first/last day half etc?

        if entity.date_finalised.is_some() {
            return Err(AppError::ValidationFailed(
                "New leave requests cannot have a finalised date".to_owned(),
            ));
        };

        Ok(())
    }

    async fn pre_update_validation(
        id: i32,
        entity: &LeaveRequest,
        ctx: &AppContext,
    ) -> Result<(), AppError> {
        Self::pre_create_and_pre_update_validation(entity)?;
        let existing_entity = Self::get_entity_by_id(id, ctx).await?;

        let leave_status_state_change_ok: bool = matches!(
            (&existing_entity.leave_status, &entity.leave_status),
            (Pending, _)
                | (MoreInfoReq, Pending)
                | (MoreInfoReq, MoreInfoReq)
                | (MoreInfoReq, Cancelled)
                | (Cancelled, Cancelled)
                | (Rejected, Rejected)
                | (Accepted, Accepted)
        );

        if !leave_status_state_change_ok {
            return Err(AppError::ValidationFailed(format!(
                "Cannot move from {} to {}",
                existing_entity.leave_status, entity.leave_status
            )));
        }

        if entity.leave_status == LeaveStatus::Accepted
            && existing_entity.leave_status != LeaveStatus::Accepted
        {
            LeaveRequestRepository::check_for_accepted_leave_for_employee_and_date_range(
                entity.employee_id,
                entity.date_from,
                entity.date_to,
                ctx,
            )
            .await?;
        }

        if (entity.leave_status == LeaveStatus::Accepted
            || entity.leave_status == LeaveStatus::Rejected)
            && entity.date_finalised.is_none()
        {
            return Err(AppError::ValidationFailed(
                "Accepted and Rejected leave requests must have a finalised date".to_string(),
            ));
        }

        Ok(())
    }

    fn pre_create_and_pre_update_validation(entity: &LeaveRequest) -> Result<(), AppError> {
        if entity.date_to < entity.date_from {
            return Err(AppError::ValidationFailed(
                "Date To cannot be before Date From".to_owned(),
            ));
        }

        if entity.date_requested > Utc::now() {
            return Err(AppError::ValidationFailed(
                "Date Requested cannot be in the future".to_owned(),
            ));
        }

        Ok(())
    }
}

impl LeaveRequestService {
    pub async fn get_my_leave(
        my_leave_request: MyLeaveRequest,
        ctx: &AppContext,
    ) -> Result<Vec<MyLeaveResponse>, AppError> {
        LeaveRequestRepository::get_my_leave(my_leave_request, ctx).await
    }

    pub async fn get_all_leave_table_data(
        all_leave_request: AllLeaveRequest,
        ctx: &AppContext,
    ) -> Result<Vec<AllLeaveResponse>, AppError> {
        LeaveRequestRepository::get_all_leave_table_data(all_leave_request, ctx).await
    }

    pub async fn update_leave_request_status(
        id: i32,
        request: LeaveRequestStatusUpdate,
        ctx: &AppContext,
    ) -> Result<LeaveRequest, AppError> {
        let mut updated_leave_request = Self::get_entity_by_id(id, ctx).await?;
        updated_leave_request.leave_status = request.leave_status;
        updated_leave_request.notes = request.notes;

        if request.leave_status == LeaveStatus::Accepted
            || request.leave_status == LeaveStatus::Rejected
        {
            updated_leave_request.date_finalised = Some(Utc::now());
        }

        updated_leave_request = Self::update_entity(id, updated_leave_request, ctx).await?;

        Ok(updated_leave_request)
    }
}
