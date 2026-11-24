//! Which leave-request changes queue an email, and that the outbox commits with the change.

use derbe_core::model::EmployeeRole::{Admin, Manager, RegularUser};
use reqwest::StatusCode;
use serde_json::Value;
use test_helpers::{
    SeededUser, TestApp, cleanup_test_app, create_leave_type, get_response_body_as_json, login_as,
    login_client, outbox_rows, post_leave_request, put_status, seed_employee, spawn_app,
};

mod test_helpers;

/// A manager, their report, and a leave type to request.
struct Team {
    manager: SeededUser,
    employee: SeededUser,
    leave_type_id: Value,
}

async fn team(app: &TestApp) -> Team {
    let (_admin, admin_client) = login_as(app, Admin).await;
    let leave_type = create_leave_type("Annual", &admin_client, app).await;
    let manager = seed_employee(app, "Mandy", Manager, None).await;
    let employee = seed_employee(app, "Emma", RegularUser, Some(manager.unid)).await;
    Team {
        manager,
        employee,
        leave_type_id: leave_type["id"].clone(),
    }
}

/// The employee submits their own request. Returns its id.
async fn submit(app: &TestApp, team: &Team) -> Value {
    let client = login_client(app, &team.employee).await;
    let response = post_leave_request(&client, app, team.employee.unid, &team.leave_type_id).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    get_response_body_as_json(response).await["id"].clone()
}

#[tokio::test]
async fn submitting_a_request_queues_one_email_for_the_manager() {
    let app = spawn_app().await;
    let team = team(&app).await;

    let id = submit(&app, &team).await;

    assert_eq!(
        outbox_rows(&app).await,
        vec![(
            format!("leave-request:{id}:created"),
            team.manager.email.clone(),
            "leave_request_submitted".to_owned(),
        )]
    );

    let payload: Value = sqlx::query_scalar("SELECT payload FROM email_outbox")
        .fetch_one(&app.ctx.pool)
        .await
        .unwrap();
    assert_eq!(payload["employee_name"], "Emma Testerson");
    assert_eq!(payload["request_id"], id);

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn a_request_for_an_employee_without_a_manager_still_saves_and_queues_nothing() {
    let app = spawn_app().await;
    let (_admin, admin_client) = login_as(&app, Admin).await;
    let leave_type = create_leave_type("Annual", &admin_client, &app).await;
    let loner = seed_employee(&app, "Lonnie", RegularUser, None).await;

    let response = post_leave_request(&admin_client, &app, loner.unid, &leave_type["id"]).await;

    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(outbox_rows(&app).await.is_empty());

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn a_manager_filing_leave_for_their_report_is_not_emailed_about_it() {
    let app = spawn_app().await;
    // An admin can file leave for anyone, so make the admin the employee's manager.
    let (boss, boss_client) = login_as(&app, Admin).await;
    let leave_type = create_leave_type("Annual", &boss_client, &app).await;
    let employee = seed_employee(&app, "Emma", RegularUser, Some(boss.unid)).await;

    let response = post_leave_request(&boss_client, &app, employee.unid, &leave_type["id"]).await;

    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(outbox_rows(&app).await.is_empty());

    cleanup_test_app(app).await;
}

async fn decision_queues_one_email_for_the_employee(status: &str) {
    let app = spawn_app().await;
    let team = team(&app).await;
    let id = submit(&app, &team).await;
    let manager_client = login_client(&app, &team.manager).await;

    let response = put_status(&manager_client, &app, &id, status).await;

    assert_eq!(response.status(), StatusCode::OK);
    let rows = outbox_rows(&app).await;
    assert_eq!(rows.len(), 2, "one submission email and one decision email");
    assert_eq!(
        rows[1],
        (
            format!("leave-request:{id}:{}", status.to_lowercase()),
            team.employee.email.clone(),
            "leave_request_decision".to_owned(),
        )
    );

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn accepting_queues_one_email_for_the_employee() {
    decision_queues_one_email_for_the_employee("Accepted").await;
}

#[tokio::test]
async fn rejecting_queues_one_email_for_the_employee() {
    decision_queues_one_email_for_the_employee("Rejected").await;
}

#[tokio::test]
async fn repeating_a_decision_queues_no_second_email() {
    let app = spawn_app().await;
    let team = team(&app).await;
    let id = submit(&app, &team).await;
    let manager_client = login_client(&app, &team.manager).await;

    assert_eq!(
        put_status(&manager_client, &app, &id, "Accepted")
            .await
            .status(),
        StatusCode::OK
    );
    let again = put_status(&manager_client, &app, &id, "Accepted").await;

    assert_eq!(again.status(), StatusCode::BAD_REQUEST);
    assert_eq!(outbox_rows(&app).await.len(), 2);

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn a_failed_outbox_insert_rolls_back_the_leave_change() {
    let app = spawn_app().await;
    let team = team(&app).await;
    let id = submit(&app, &team).await;

    // From now on every outbox insert fails. NOT VALID leaves the existing row alone.
    sqlx::query("ALTER TABLE email_outbox ADD CONSTRAINT reject_all CHECK (false) NOT VALID")
        .execute(&app.ctx.pool)
        .await
        .unwrap();

    // A new request must not be saved without its email...
    let employee_client = login_client(&app, &team.employee).await;
    let response = post_leave_request(
        &employee_client,
        &app,
        team.employee.unid,
        &team.leave_type_id,
    )
    .await;
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let requests: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM leave_requests")
        .fetch_one(&app.ctx.pool)
        .await
        .unwrap();
    assert_eq!(
        requests, 1,
        "only the request created before the constraint"
    );

    // ...and a decision must not be saved without its email.
    let manager_client = login_client(&app, &team.manager).await;
    let response = put_status(&manager_client, &app, &id, "Accepted").await;
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let status: String =
        sqlx::query_scalar("SELECT leave_status::text FROM leave_requests WHERE id = $1")
            .bind(id.as_i64().unwrap() as i32)
            .fetch_one(&app.ctx.pool)
            .await
            .unwrap();
    assert_eq!(status, "Pending");

    cleanup_test_app(app).await;
}
