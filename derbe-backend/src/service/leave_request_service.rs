use crate::{model::leave_request::LeaveRequest, repository::leave_request_respository::LeaveRequestRepository};

use super::DerbeService;

pub struct LeaveRequestService {}

impl DerbeService<LeaveRequest, LeaveRequestRepository> for LeaveRequestService {}