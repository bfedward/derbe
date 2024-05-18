use crate::{
    dto::{request::leave_request::MyLeaveRequest, response::leave_request::MyLeaveResponse},
    error::AppError,
    http::AppContext,
    model::leave_request::LeaveRequest,
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
}

impl LeaveRequestService {
    pub async fn get_my_leave(
        my_leave_request: MyLeaveRequest,
        ctx: &AppContext,
    ) -> Result<Vec<MyLeaveResponse>, AppError> {
        LeaveRequestRepository::get_my_leave(my_leave_request, ctx).await
    }
}
