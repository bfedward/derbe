use derbe_shared::dto::response::leave_type::LeaveTypeResponse;

pub struct LeaveType {
    pub id: i32,
    pub name: String,
    pub default_quota: Option<f64>,
    pub colour: String,
    pub can_book_in_future: bool,
    pub show_in_others_days: bool,
    pub default_in_others_days: bool,
    pub default_in_leave_request: bool,
    pub show_in_approve_days: bool,
}

impl From<LeaveType> for LeaveTypeResponse {
    fn from(value: LeaveType) -> Self {
        Self {
            id: value.id,
            name: value.name,
            default_quota: value.default_quota,
            colour: value.colour,
            can_book_in_future: value.can_book_in_future,
            show_in_others_days: value.show_in_others_days,
            default_in_others_days: value.default_in_others_days,
            default_in_leave_request: value.default_in_leave_request,
            show_in_approve_days: value.show_in_approve_days,
        }
    }
}
