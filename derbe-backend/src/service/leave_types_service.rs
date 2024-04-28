use crate::{
    model::leave_type::LeaveType, repository::leave_types_repository::LeaveTypesRepository,
};

use super::DerbeService;

pub struct LeaveTypesService {}

impl DerbeService<LeaveType, LeaveTypesRepository> for LeaveTypesService {}
