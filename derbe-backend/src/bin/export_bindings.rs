use derbe_backend::features::{
    auth::model::CurrentUser,
    divisions::dto::{
        request::{DivisionCreate, DivisionUpdate},
        response::DivisionResponse,
    },
    employees::{
        dto::{
            request::{EmployeeCreate, EmployeeUpdate},
            response::{EmployeeDetailsResponse, EmployeeResponse},
        },
        model::{EmployeeRole, EmploymentType},
    },
    leave_requests::{
        dto::{
            request::{
                AllLeaveRequest, LeaveRequestCreate, LeaveRequestStatusUpdate, LeaveRequestUpdate,
                LeaveRequestValues, MyLeaveRequest,
            },
            response::{
                AllLeaveInnerResponse, AllLeaveResponse, LeaveRequestDetailsResponse,
                LeaveRequestResponse, MyLeaveResponse, TeamLeaveResponse,
            },
        },
        model::{HalfDayOptions, LeaveStatus},
    },
    leave_types::dto::{
        request::{LeaveTypeCreate, LeaveTypeUpdate},
        response::LeaveTypeResponse,
    },
    locations::dto::{
        request::{LocationCreate, LocationUpdate},
        response::LocationResponse,
    },
};
use ts_rs::TS;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    CurrentUser::export_all()?;
    EmployeeRole::export_all()?;
    DivisionCreate::export_all()?;
    DivisionUpdate::export_all()?;
    EmployeeCreate::export_all()?;
    EmployeeUpdate::export_all()?;
    LocationCreate::export_all()?;
    LocationUpdate::export_all()?;
    LeaveTypeCreate::export_all()?;
    LeaveTypeUpdate::export_all()?;
    LeaveRequestDetailsResponse::export_all()?;
    LeaveRequestCreate::export_all()?;
    LeaveRequestValues::export_all()?;
    LeaveRequestUpdate::export_all()?;
    AllLeaveRequest::export_all()?;
    MyLeaveRequest::export_all()?;
    TeamLeaveResponse::export_all()?;
    LeaveRequestStatusUpdate::export_all()?;
    DivisionResponse::export_all()?;
    EmployeeResponse::export_all()?;
    EmployeeDetailsResponse::export_all()?;
    LeaveRequestResponse::export_all()?;
    AllLeaveResponse::export_all()?;
    AllLeaveInnerResponse::export_all()?;
    MyLeaveResponse::export_all()?;
    LeaveTypeResponse::export_all()?;
    LocationResponse::export_all()?;
    EmploymentType::export_all()?;
    LeaveStatus::export_all()?;
    HalfDayOptions::export_all()?;
    Ok(())
}
