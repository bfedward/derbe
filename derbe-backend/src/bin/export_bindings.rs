use derbe_backend::{
    dto::{
        request::{
            division::{DivisionCreate, DivisionUpdate},
            employee::{EmployeeCreate, EmployeeUpdate},
            leave_request::{
                AllLeaveRequest, LeaveRequestCreate, LeaveRequestStatusUpdate,
                LeaveRequestUpdate, LeaveRequestValues, MyLeaveRequest,
            },
            leave_type::{LeaveTypeCreate, LeaveTypeUpdate},
            location::{LocationCreate, LocationUpdate},
        },
        response::{
            division::DivisionResponse,
            employee::{EmployeeDetailsResponse, EmployeeResponse},
            leave_request::{
                AllLeaveInnerResponse, AllLeaveResponse, LeaveRequestResponse, MyLeaveResponse,
            },
            leave_type::LeaveTypeResponse,
            location::LocationResponse,
        },
    },
    enums::{EmploymentType, HalfDayOptions, LeaveStatus},
};
use ts_rs::TS;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    DivisionCreate::export_all()?;
    DivisionUpdate::export_all()?;
    EmployeeCreate::export_all()?;
    EmployeeUpdate::export_all()?;
    LocationCreate::export_all()?;
    LocationUpdate::export_all()?;
    LeaveTypeCreate::export_all()?;
    LeaveTypeUpdate::export_all()?;
    LeaveRequestCreate::export_all()?;
    LeaveRequestValues::export_all()?;
    LeaveRequestUpdate::export_all()?;
    AllLeaveRequest::export_all()?;
    MyLeaveRequest::export_all()?;
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
