//! GET /teamLeave: the pending requests the logged-in user can decide on.

use derbe_core::model::EmployeeRole::{Admin, Demo, Manager, RegularUser};
use reqwest::{Client, Response, StatusCode};
use serde_json::Value;
use test_helpers::{
    SeededUser, TestApp, cleanup_test_app, create_leave_type, get_response_body_as_json,
    login_client, post_leave_request, put_status, seed_employee, spawn_app,
};

mod test_helpers;

async fn get_team_leave(client: &Client, app: &TestApp) -> Response {
    client
        .get(format!("{}/teamLeave", app.address))
        .send()
        .await
        .expect("Failed to get team leave")
}

/// Team Leave as (employee_name, leave_request_id) pairs, in the order returned.
async fn team_leave_rows(client: &Client, app: &TestApp) -> Vec<(String, i64)> {
    let response = get_team_leave(client, app).await;
    assert_eq!(response.status(), StatusCode::OK);

    get_response_body_as_json(response)
        .await
        .as_array()
        .expect("team leave should be a JSON array")
        .iter()
        .map(|row| {
            (
                row["employee_name"].as_str().unwrap().to_owned(),
                row["leave_request_id"].as_i64().unwrap(),
            )
        })
        .collect()
}

/// The employee files their own pending request. Returns its id.
async fn submit(app: &TestApp, employee: &SeededUser, leave_type_id: &Value) -> i64 {
    let client = login_client(app, employee).await;
    let response = post_leave_request(&client, app, employee.unid, leave_type_id).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    get_response_body_as_json(response).await["id"]
        .as_i64()
        .unwrap()
}

/// Two managers with one report each, plus an admin and a leave type.
struct Org {
    admin: Client,
    leave_type_id: Value,
    mandy: SeededUser,
    otto: SeededUser,
    emma: SeededUser,  // reports to Mandy
    oscar: SeededUser, // reports to Otto
}

async fn org(app: &TestApp) -> Org {
    let ada = seed_employee(app, "Ada", Admin, None).await;
    let admin = login_client(app, &ada).await;
    let leave_type = create_leave_type("Annual", &admin, app).await;

    let mandy = seed_employee(app, "Mandy", Manager, None).await;
    let otto = seed_employee(app, "Otto", Manager, None).await;
    let emma = seed_employee(app, "Emma", RegularUser, Some(mandy.unid)).await;
    let oscar = seed_employee(app, "Oscar", RegularUser, Some(otto.unid)).await;

    Org {
        admin,
        leave_type_id: leave_type["id"].clone(),
        mandy,
        otto,
        emma,
        oscar,
    }
}

fn id_value(id: i64) -> Value {
    Value::from(id)
}

#[tokio::test]
async fn a_manager_sees_only_the_pending_requests_of_their_direct_reports() {
    let app = spawn_app().await;
    let org = org(&app).await;
    let emmas = submit(&app, &org.emma, &org.leave_type_id).await;
    let oscars = submit(&app, &org.oscar, &org.leave_type_id).await;

    let mandy = login_client(&app, &org.mandy).await;
    let otto = login_client(&app, &org.otto).await;

    assert_eq!(
        team_leave_rows(&mandy, &app).await,
        vec![("Emma Testerson".to_owned(), emmas)]
    );
    assert_eq!(
        team_leave_rows(&otto, &app).await,
        vec![("Oscar Testerson".to_owned(), oscars)]
    );

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn an_admin_sees_pending_requests_from_everyone() {
    let app = spawn_app().await;
    let org = org(&app).await;
    // Someone with no manager at all must still show up for an admin.
    let lonnie = seed_employee(&app, "Lonnie", RegularUser, None).await;

    let emmas = submit(&app, &org.emma, &org.leave_type_id).await;
    let oscars = submit(&app, &org.oscar, &org.leave_type_id).await;
    let lonnies = submit(&app, &lonnie, &org.leave_type_id).await;

    let mut ids: Vec<i64> = team_leave_rows(&org.admin, &app)
        .await
        .into_iter()
        .map(|(_, id)| id)
        .collect();
    ids.sort();

    let mut expected = vec![emmas, oscars, lonnies];
    expected.sort();
    assert_eq!(ids, expected);

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn a_manager_does_not_see_their_own_pending_request() {
    let app = spawn_app().await;
    let org = org(&app).await;
    submit(&app, &org.mandy, &org.leave_type_id).await;

    let mandy = login_client(&app, &org.mandy).await;

    assert!(team_leave_rows(&mandy, &app).await.is_empty());

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn decided_and_cancelled_requests_are_not_listed() {
    let app = spawn_app().await;
    let org = org(&app).await;
    let pat = seed_employee(&app, "Pat", RegularUser, Some(org.mandy.unid)).await;
    let acey = seed_employee(&app, "Acey", RegularUser, Some(org.mandy.unid)).await;
    let remy = seed_employee(&app, "Remy", RegularUser, Some(org.mandy.unid)).await;
    let cass = seed_employee(&app, "Cass", RegularUser, Some(org.mandy.unid)).await;

    let pats = submit(&app, &pat, &org.leave_type_id).await;
    let aceys = submit(&app, &acey, &org.leave_type_id).await;
    let remys = submit(&app, &remy, &org.leave_type_id).await;
    let cass_request = submit(&app, &cass, &org.leave_type_id).await;

    let mandy = login_client(&app, &org.mandy).await;
    assert_eq!(
        put_status(&mandy, &app, &id_value(aceys), "Accepted")
            .await
            .status(),
        StatusCode::OK
    );
    assert_eq!(
        put_status(&mandy, &app, &id_value(remys), "Rejected")
            .await
            .status(),
        StatusCode::OK
    );
    let cass_client = login_client(&app, &cass).await;
    assert_eq!(
        put_status(&cass_client, &app, &id_value(cass_request), "Cancelled")
            .await
            .status(),
        StatusCode::OK
    );

    assert_eq!(
        team_leave_rows(&mandy, &app).await,
        vec![("Pat Testerson".to_owned(), pats)]
    );

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn requests_from_deleted_employees_are_not_listed() {
    let app = spawn_app().await;
    let org = org(&app).await;
    submit(&app, &org.emma, &org.leave_type_id).await;

    sqlx::query("UPDATE employees SET deleted_at = now() WHERE unid = $1")
        .bind(org.emma.unid)
        .execute(&app.ctx.pool)
        .await
        .unwrap();

    let mandy = login_client(&app, &org.mandy).await;
    assert!(team_leave_rows(&mandy, &app).await.is_empty());

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn regular_users_and_demo_users_are_forbidden() {
    let app = spawn_app().await;
    let org = org(&app).await;
    let demo = seed_employee(&app, "Dee", Demo, None).await;

    let emma = login_client(&app, &org.emma).await;
    let demo = login_client(&app, &demo).await;

    assert_eq!(
        get_team_leave(&emma, &app).await.status(),
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        get_team_leave(&demo, &app).await.status(),
        StatusCode::FORBIDDEN
    );

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn team_leave_requires_a_session() {
    let app = spawn_app().await;

    let response = get_team_leave(&Client::new(), &app).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    cleanup_test_app(app).await;
}
