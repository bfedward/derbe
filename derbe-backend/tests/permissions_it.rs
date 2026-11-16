//! One integration test per permission rule, through the real router,
//! with a real session cookie per role.

use derbe_backend::features::employees::model::EmployeeRole::{Admin, Demo, Manager, RegularUser};
use reqwest::{Client, Response, StatusCode};
use serde_json::{json, Value};
use test_helpers::{
    cleanup_test_app, create_division, create_leave_type, get_response_body_as_json, login_as,
    login_client, seed_employee, spawn_app, TestApp,
};
use uuid::Uuid;

mod test_helpers;

fn employee_body(division: &Value, location: &Value, email: &str) -> Value {
    json!({
        "first_name": "Newcomer",
        "last_name": "Testerson",
        "job_title": "Tester",
        "division_id": division["id"],
        "employment_type": "Permanent",
        "location_id": location["id"],
        "manager_unid": null,
        "email": email,
    })
}

async fn post_leave_request(
    client: &Client,
    app: &TestApp,
    employee_unid: Uuid,
    leave_type_id: &Value,
) -> Response {
    let body = json!({
        "employee_unid": employee_unid,
        "leave_type_id": leave_type_id,
        "leave_status": "Pending",
        "values": {
            "date_from": "2030-05-20",
            "date_to": "2030-05-24",
            "first_day_half": false,
            "last_day_half": false,
            "total_duration": 5.0,
        },
        "date_requested": "2030-05-11T13:48:48.739+00:00",
        "date_finalised": null,
        "notes": null
    });

    client
        .post(format!("{}/leaveRequests", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to create leave request")
}

async fn put_status(client: &Client, app: &TestApp, id: &Value, status: &str) -> Response {
    client
        .put(format!("{}/leaveRequests/{}/status", app.address, id))
        .json(&json!({ "leave_status": status, "notes": null }))
        .send()
        .await
        .expect("Failed to update status")
}

/// A pending leave request owned by `owner`, created by an admin.
async fn pending_leave_request(
    app: &TestApp,
    admin: &Client,
    owner: Uuid,
    leave_type_id: &Value,
) -> Value {
    let response = post_leave_request(admin, app, owner, leave_type_id).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    get_response_body_as_json(response).await
}

// Authentication

#[tokio::test]
async fn protected_routes_return_401_without_a_session() {
    let (app, config) = spawn_app().await;
    let client = Client::new();

    for path in ["/divisions", "/locations", "/employees", "/leaveTypes"] {
        let response = client
            .get(format!("{}{}", app.address, path))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "GET {path}");
    }

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn login_fails_with_a_wrong_password() {
    let (app, config) = spawn_app().await;
    let user = seed_employee(&app, "Wrongpass", RegularUser, None).await;

    let response = Client::new()
        .post(format!("{}/login", app.address))
        .json(&json!({ "email": user.email, "password": "not-the-password" }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn a_deleted_employee_cannot_log_in() {
    let (app, config) = spawn_app().await;
    let user = seed_employee(&app, "Leaver", RegularUser, None).await;

    sqlx::query("UPDATE employees SET deleted_at = now() WHERE unid = $1")
        .bind(user.unid)
        .execute(&app.ctx.pool)
        .await
        .unwrap();

    let response = Client::new()
        .post(format!("{}/login", app.address))
        .json(&json!({ "email": user.email, "password": "test-password" }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn me_returns_the_role_of_the_logged_in_employee() {
    let (app, config) = spawn_app().await;

    for role in [Admin, Manager, RegularUser, Demo] {
        let (user, client) = login_as(&app, role).await;
        let response = client
            .get(format!("{}/me", app.address))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let me = get_response_body_as_json(response).await;
        assert_eq!(me["role"], role.to_string());
        assert_eq!(me["unid"], user.unid.to_string());
    }

    cleanup_test_app(&config.database, app).await;
}

// Divisions, locations and leave types: Admin creates, every role reads

#[tokio::test]
async fn only_admin_can_create_divisions() {
    let (app, config) = spawn_app().await;

    for (role, expected) in [
        (Admin, StatusCode::CREATED),
        (Manager, StatusCode::FORBIDDEN),
        (RegularUser, StatusCode::FORBIDDEN),
        (Demo, StatusCode::FORBIDDEN),
    ] {
        let (_user, client) = login_as(&app, role).await;
        let response = client
            .post(format!("{}/divisions", app.address))
            .json(&json!({ "name": format!("Division of {role}") }))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), expected, "{role} creating a division");
    }

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn only_admin_can_create_locations() {
    let (app, config) = spawn_app().await;

    for (role, expected) in [
        (Admin, StatusCode::CREATED),
        (Manager, StatusCode::FORBIDDEN),
        (RegularUser, StatusCode::FORBIDDEN),
        (Demo, StatusCode::FORBIDDEN),
    ] {
        let (_user, client) = login_as(&app, role).await;
        let response = client
            .post(format!("{}/locations", app.address))
            .json(&json!({ "name": format!("Location of {role}") }))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), expected, "{role} creating a location");
    }

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn only_admin_can_create_leave_types() {
    let (app, config) = spawn_app().await;

    for (role, expected) in [
        (Admin, StatusCode::CREATED),
        (Manager, StatusCode::FORBIDDEN),
        (RegularUser, StatusCode::FORBIDDEN),
        (Demo, StatusCode::FORBIDDEN),
    ] {
        let (_user, client) = login_as(&app, role).await;
        let response = client
            .post(format!("{}/leaveTypes", app.address))
            .json(&json!({
                "name": format!("Type of {role}"),
                "default_quota": 10.0,
                "colour": "#FFFFFF",
                "can_book_in_future": true,
                "show_in_others_days": true,
                "default_in_others_days": true,
                "default_in_leave_request": true,
                "show_in_approve_days": true
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), expected, "{role} creating a leave type");
    }

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn every_role_can_read_divisions_locations_and_leave_types() {
    let (app, config) = spawn_app().await;

    for role in [Admin, Manager, RegularUser, Demo] {
        let (_user, client) = login_as(&app, role).await;
        for path in ["/divisions", "/locations", "/leaveTypes"] {
            let response = client
                .get(format!("{}{}", app.address, path))
                .send()
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{role} reading {path}");
        }
    }

    cleanup_test_app(&config.database, app).await;
}

// Employees

#[tokio::test]
async fn manager_creates_employees_but_regular_user_and_demo_cannot() {
    let (app, config) = spawn_app().await;
    let (_admin, admin_client) = login_as(&app, Admin).await;
    let division = create_division("Ops", &admin_client, &app).await;
    let location = test_helpers::create_location("HQ", &admin_client, &app).await;

    for (role, expected) in [
        (Manager, StatusCode::CREATED),
        (RegularUser, StatusCode::FORBIDDEN),
        (Demo, StatusCode::FORBIDDEN),
    ] {
        let (_user, client) = login_as(&app, role).await;
        let response = client
            .post(format!("{}/employees", app.address))
            .json(&employee_body(
                &division,
                &location,
                &format!("new-{}@derbe.test", role.to_string().to_lowercase()),
            ))
            .send()
            .await
            .unwrap();
        let status = response.status();
        let body = response.text().await.unwrap();
        assert_eq!(status, expected, "{role} creating an employee: {body}");
    }

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn demo_cannot_update_an_employee_but_can_list_them() {
    let (app, config) = spawn_app().await;
    let (demo, client) = login_as(&app, Demo).await;

    let list = client
        .get(format!("{}/employees", app.address))
        .send()
        .await
        .unwrap();
    assert_eq!(list.status(), StatusCode::OK);

    let update = client
        .put(format!("{}/employees/{}", app.address, demo.unid))
        .json(&json!({
            "first_name": "Renamed",
            "last_name": "Testerson",
            "job_title": "Tester",
            "division_id": 1,
            "employment_type": "Permanent",
            "location_id": 1,
            "manager_unid": null,
            "email": demo.email,
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(update.status(), StatusCode::FORBIDDEN);

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn regular_user_cannot_update_someone_else() {
    let (app, config) = spawn_app().await;
    let (_user, client) = login_as(&app, RegularUser).await;
    let other = seed_employee(&app, "Colleague", RegularUser, None).await;

    let response = client
        .put(format!("{}/employees/{}", app.address, other.unid))
        .json(&json!({
            "first_name": "Renamed",
            "last_name": "Testerson",
            "job_title": "Tester",
            "division_id": 1,
            "employment_type": "Permanent",
            "location_id": 1,
            "manager_unid": null,
            "email": other.email,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    cleanup_test_app(&config.database, app).await;
}

// Leave requests

#[tokio::test]
async fn regular_user_creates_their_own_leave_request_but_not_someone_elses() {
    let (app, config) = spawn_app().await;
    let (_admin, admin_client) = login_as(&app, Admin).await;
    let leave_type = create_leave_type("Annual", &admin_client, &app).await;
    let (user, client) = login_as(&app, RegularUser).await;
    let other = seed_employee(&app, "Colleague", RegularUser, None).await;

    let own = post_leave_request(&client, &app, user.unid, &leave_type["id"]).await;
    assert_eq!(own.status(), StatusCode::CREATED);

    let someone_elses = post_leave_request(&client, &app, other.unid, &leave_type["id"]).await;
    assert_eq!(someone_elses.status(), StatusCode::FORBIDDEN);

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn demo_cannot_create_a_leave_request() {
    let (app, config) = spawn_app().await;
    let (_admin, admin_client) = login_as(&app, Admin).await;
    let leave_type = create_leave_type("Annual", &admin_client, &app).await;
    let (demo, client) = login_as(&app, Demo).await;

    let response = post_leave_request(&client, &app, demo.unid, &leave_type["id"]).await;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn manager_approves_a_report_but_not_their_own_leave() {
    let (app, config) = spawn_app().await;
    let (_admin, admin_client) = login_as(&app, Admin).await;
    let leave_type = create_leave_type("Annual", &admin_client, &app).await;
    let manager = seed_employee(&app, "Boss", Manager, None).await;
    let report = seed_employee(&app, "Report", RegularUser, Some(manager.unid)).await;
    let manager_client = login_client(&app, &manager).await;

    let reports_leave =
        pending_leave_request(&app, &admin_client, report.unid, &leave_type["id"]).await;
    let own_leave =
        pending_leave_request(&app, &admin_client, manager.unid, &leave_type["id"]).await;

    let approve_report = put_status(&manager_client, &app, &reports_leave["id"], "Accepted").await;
    assert_eq!(approve_report.status(), StatusCode::OK);

    let approve_own = put_status(&manager_client, &app, &own_leave["id"], "Accepted").await;
    assert_eq!(approve_own.status(), StatusCode::FORBIDDEN);

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn manager_cannot_approve_the_leave_of_someone_who_is_not_a_report() {
    let (app, config) = spawn_app().await;
    let (_admin, admin_client) = login_as(&app, Admin).await;
    let leave_type = create_leave_type("Annual", &admin_client, &app).await;
    let (_manager, manager_client) = login_as(&app, Manager).await;
    let stranger = seed_employee(&app, "Stranger", RegularUser, None).await;

    let leave = pending_leave_request(&app, &admin_client, stranger.unid, &leave_type["id"]).await;
    let response = put_status(&manager_client, &app, &leave["id"], "Accepted").await;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn regular_user_cannot_approve_even_their_own_leave() {
    let (app, config) = spawn_app().await;
    let (_admin, admin_client) = login_as(&app, Admin).await;
    let leave_type = create_leave_type("Annual", &admin_client, &app).await;
    let (user, client) = login_as(&app, RegularUser).await;

    let leave = pending_leave_request(&app, &admin_client, user.unid, &leave_type["id"]).await;
    let response = put_status(&client, &app, &leave["id"], "Accepted").await;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn regular_user_can_cancel_their_own_leave_request() {
    let (app, config) = spawn_app().await;
    let (_admin, admin_client) = login_as(&app, Admin).await;
    let leave_type = create_leave_type("Annual", &admin_client, &app).await;
    let (user, client) = login_as(&app, RegularUser).await;

    let leave = pending_leave_request(&app, &admin_client, user.unid, &leave_type["id"]).await;
    let response = put_status(&client, &app, &leave["id"], "Cancelled").await;

    assert_eq!(response.status(), StatusCode::OK);

    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn a_rejected_leave_request_cannot_go_back_to_pending() {
    let (app, config) = spawn_app().await;
    let (_admin, admin_client) = login_as(&app, Admin).await;
    let leave_type = create_leave_type("Annual", &admin_client, &app).await;
    let (user, client) = login_as(&app, RegularUser).await;

    let leave = pending_leave_request(&app, &admin_client, user.unid, &leave_type["id"]).await;
    let rejected = put_status(&admin_client, &app, &leave["id"], "Rejected").await;
    assert_eq!(rejected.status(), StatusCode::OK);

    let back_to_pending = put_status(&client, &app, &leave["id"], "Pending").await;

    assert_eq!(back_to_pending.status(), StatusCode::BAD_REQUEST);

    cleanup_test_app(&config.database, app).await;
}
