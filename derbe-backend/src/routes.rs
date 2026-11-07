use axum::{
    routing::{get, post, put},
    Router,
};

use crate::{
    features::{
        divisions::handlers as division_handlers, employees::handlers as employee_handlers,
        leave_requests::handlers as leave_request_handlers,
        leave_types::handlers as leave_type_handlers, locations::handlers as location_handlers,
    },
    http::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/divisions",
            get(division_handlers::list_divisions).post(division_handlers::create_division),
        )
        .route("/divisions/{id}", get(division_handlers::get_division))
        .route(
            "/employees",
            get(employee_handlers::list_employees).post(employee_handlers::create_employee),
        )
        .route(
            "/employees/{id}",
            get(employee_handlers::get_employee).put(employee_handlers::update_employee),
        )
        .route(
            "/employees/detailsTable",
            get(employee_handlers::get_employee_details_table),
        )
        .route(
            "/leaveTypes",
            get(leave_type_handlers::list_leave_types).post(leave_type_handlers::create_leave_type),
        )
        .route("/leaveTypes/{id}", get(leave_type_handlers::get_leave_type))
        .route(
            "/locations",
            get(location_handlers::list_locations).post(location_handlers::create_location),
        )
        .route("/locations/{id}", get(location_handlers::get_location))
        .route(
            "/leaveRequests",
            post(leave_request_handlers::create_leave_request),
        )
        .route(
            "/leaveRequests/{id}",
            get(leave_request_handlers::get_leave_request)
                .put(leave_request_handlers::update_leave_request),
        )
        .route(
            "/leaveRequests/{id}/status",
            put(leave_request_handlers::update_leave_request_status),
        )
        .route("/myLeave", post(leave_request_handlers::get_my_leave))
        .route("/allLeave", post(leave_request_handlers::get_all_leave))
}
