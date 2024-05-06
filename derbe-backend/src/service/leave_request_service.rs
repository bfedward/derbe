use crate::{dto::request::leave_request::MyLeaveRequest, error::AppError, http::AppContext, model::leave_request::LeaveRequest, repository::leave_request_respository::LeaveRequestRepository};

use super::DerbeService;

pub struct LeaveRequestService {}

impl DerbeService<LeaveRequest, LeaveRequestRepository> for LeaveRequestService {}

impl LeaveRequestService {

    pub async fn get_my_leave(my_leave_request: MyLeaveRequest, ctx: &AppContext) -> Result<Vec<LeaveRequest>, AppError> {
        LeaveRequestRepository::get_my_leave(my_leave_request, ctx).await
    }

}