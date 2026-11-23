use derbe_backend::features::employees::model::EmployeeRole::{Admin, Manager, RegularUser};
use reqwest::{Client, Response, StatusCode};
use serde_json::Value;
use test_helpers::{
    cleanup_test_app, create_division, create_employee, create_location, do_post, employee_body,
    get_response_body_as_json, login_client, seed_employee, TestApp,
};
use uuid::Uuid;

mod test_helpers;
struct Places {
    division: Value,
    location: Value,
}

async fn places(app: &TestApp) -> Places {
    let ada = seed_employee(app, "Ada", Admin, None).await;
    let admin = login_client(app, &ada).await;
    Places {
        division: create_division("Division", &admin, app).await,
        location: create_location("Location", &admin, app).await,
    }
}

/// An employee body with an optional role and a manager.
fn with_role(mut body: Value, role: Option<&str>, manager_unid: Option<Uuid>) -> Value {
    if let Some(role) = role {
        body["role"] = role.into();
    }
    body["manager_unid"] = manager_unid.map(|unid| unid.to_string()).into();
    body
}

async fn create_as(
    client: &Client,
    app: &TestApp,
    places: &Places,
    email: &str,
    role: Option<&str>,
    manager_unid: Option<Uuid>,
) -> Response {
    let body = with_role(
        employee_body("New", email, &places.division, &places.location),
        role,
        manager_unid,
    );
    do_post(
        client,
        format!("{}/employees", app.address),
        body,
        "Failed to post employee",
    )
    .await
}

#[tokio::test]
async fn get_employees() {
    let app = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(
        &app,
        derbe_backend::features::employees::model::EmployeeRole::Admin,
    )
    .await;

    println!("{}", &app.address);

    let response = client
        .get(format!("{}/employees", &app.address))
        .send()
        .await
        .expect("Failed to get employees");

    assert!(response.status().is_success());

    test_helpers::cleanup_test_app(app).await;
}

#[tokio::test]
async fn get_employee() {
    let app = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(
        &app,
        derbe_backend::features::employees::model::EmployeeRole::Admin,
    )
    .await;

    let first_name = "Dorothy";

    let create_response: serde_json::Value = create_employee(first_name, &client, &app).await;
    let id = create_response["employee_unid"].as_str().unwrap();

    let get_response = client
        .get(format!("{}/employees/{}", app.address, id))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(get_response.status(), StatusCode::OK);
    let response_text: serde_json::Value = get_response_body_as_json(get_response).await;
    assert_eq!(response_text["employee_unid"], id);
    assert_eq!(response_text["first_name"], first_name);
    assert_eq!(response_text["last_name"], create_response["last_name"]);
    assert_eq!(response_text["job_title"], create_response["job_title"]);
    assert_eq!(response_text["division_id"], create_response["division_id"]);
    assert_eq!(
        response_text["employment_type"],
        create_response["employment_type"]
    );
    assert_eq!(response_text["location_id"], create_response["location_id"]);
    assert_eq!(
        response_text["manager_unid"],
        create_response["manager_unid"]
    );
    assert_eq!(response_text["email"], create_response["email"]);
    assert_eq!(response_text["role"], "RegularUser");
    assert_eq!(response_text["role"], create_response["role"]);

    // Clean up the test application
    cleanup_test_app(app).await;
}

#[tokio::test]
async fn cannot_get_non_existant_employee() {
    let app = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(
        &app,
        derbe_backend::features::employees::model::EmployeeRole::Admin,
    )
    .await;

    let get_response = client
        .get(format!("{}/employees/{}", app.address, uuid::Uuid::nil()))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);

    // Clean up the test application
    cleanup_test_app(app).await;
}

#[tokio::test]
async fn create_employee_success() {
    let app = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(
        &app,
        derbe_backend::features::employees::model::EmployeeRole::Admin,
    )
    .await;
    let first_name = "Dorothy";

    let response = create_employee(first_name, &client, &app).await;

    assert!(
        uuid::Uuid::parse_str(response["employee_unid"].as_str().unwrap()).is_ok(),
        "employee_unid should be a UUID"
    );

    // Assert that the "first_name" field matches the input
    assert_eq!(response["first_name"], first_name);

    // Clean up the test application
    cleanup_test_app(app).await;
}

#[tokio::test]
async fn cannot_create_employee_with_blank_first_name() {
    let app = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(
        &app,
        derbe_backend::features::employees::model::EmployeeRole::Admin,
    )
    .await;

    let division = create_division("Division", &client, &app).await;
    let location = create_location("Location", &client, &app).await;

    let body = employee_body("", "test@derbe.com", &division, &location); // first name is blank

    let response = do_post(
        &client,
        format!("{}/employees", app.address),
        body,
        "Failed to create employee",
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response_text: String = test_helpers::get_response_body(response).await;

    assert!(response_text.contains("first_name"));

    // Clean up the test application
    cleanup_test_app(app).await;
}

#[tokio::test]
async fn an_employees_manager_must_be_an_admin_or_a_manager() {
    let app = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(&app, Admin).await;
    let division = create_division("Division", &client, &app).await;
    let location = create_location("Location", &client, &app).await;
    let regular = test_helpers::seed_employee(&app, "Reggie", RegularUser, None).await;
    let boss = test_helpers::seed_employee(&app, "Boss", Manager, None).await;
    let url = format!("{}/employees", app.address);

    let mut under_regular = employee_body("Ann", "ann@derbe.test", &division, &location);
    under_regular["manager_unid"] = regular.unid.to_string().into();
    let mut under_manager = employee_body("Bea", "bea@derbe.test", &division, &location);
    under_manager["manager_unid"] = boss.unid.to_string().into();

    let rejected = do_post(&client, url.clone(), under_regular, "post").await;
    let accepted = do_post(&client, url, under_manager, "post").await;

    assert_eq!(rejected.status(), StatusCode::BAD_REQUEST);
    assert_eq!(accepted.status(), StatusCode::CREATED);

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn employee_email_is_stored_lowercase() {
    let app = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(
        &app,
        derbe_backend::features::employees::model::EmployeeRole::Admin,
    )
    .await;

    let division = create_division("Division", &client, &app).await;
    let location = create_location("Location", &client, &app).await;
    let body = employee_body("Dorothy", "  Dorothy@Derbe.COM ", &division, &location);

    let response = do_post(
        &client,
        format!("{}/employees", app.address),
        body,
        "Failed to create employee",
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let response_json = get_response_body_as_json(response).await;

    assert_eq!(response_json["employee"]["email"], "dorothy@derbe.com");

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn cannot_create_employee_with_invalid_email() {
    let app = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(
        &app,
        derbe_backend::features::employees::model::EmployeeRole::Admin,
    )
    .await;

    let division = create_division("Division", &client, &app).await;
    let location = create_location("Location", &client, &app).await;
    let body = employee_body("Dorothy", "not-an-email", &division, &location);

    let response = do_post(
        &client,
        format!("{}/employees", app.address),
        body,
        "Failed to create employee",
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response_text: String = test_helpers::get_response_body(response).await;

    assert!(response_text.contains("email"));

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn cannot_create_two_employees_with_the_same_email() {
    let app = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(&app, Admin).await;
    let division = create_division("Division", &client, &app).await;
    let location = create_location("Location", &client, &app).await;
    let url = format!("{}/employees", app.address);

    let first = do_post(
        &client,
        url.clone(),
        employee_body("Ann", "same@derbe.test", &division, &location),
        "post",
    )
    .await;
    // Different case and surrounding spaces still count as the same address.
    let second = do_post(
        &client,
        url,
        employee_body("Bea", "  SAME@derbe.test ", &division, &location),
        "post",
    )
    .await;

    assert_eq!(first.status(), StatusCode::CREATED);
    assert_eq!(second.status(), StatusCode::CONFLICT);
    assert_eq!(
        get_response_body_as_json(second).await["error"],
        "An employee with this email address already exists"
    );

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn cannot_update_an_employee_to_another_employees_email() {
    let app = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(&app, Admin).await;
    let division = create_division("Division", &client, &app).await;
    let location = create_location("Location", &client, &app).await;
    let url = format!("{}/employees", app.address);
    do_post(
        &client,
        url.clone(),
        employee_body("Ann", "ann@derbe.test", &division, &location),
        "post",
    )
    .await;
    let bea = get_response_body_as_json(
        do_post(
            &client,
            url.clone(),
            employee_body("Bea", "bea@derbe.test", &division, &location),
            "post",
        )
        .await,
    )
    .await["employee"]
        .take();
    let bea_url = format!("{url}/{}", bea["employee_unid"].as_str().unwrap());

    let taken = client
        .put(&bea_url)
        .json(&employee_body(
            "Bea",
            "ann@derbe.test",
            &division,
            &location,
        ))
        .send()
        .await
        .unwrap();
    let unchanged = client
        .put(&bea_url)
        .json(&employee_body(
            "Beatrice",
            "bea@derbe.test",
            &division,
            &location,
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(taken.status(), StatusCode::CONFLICT);
    assert_eq!(
        unchanged.status(),
        StatusCode::OK,
        "keeping your own email is not a conflict"
    );

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn a_deleted_employees_email_can_be_reused() {
    let app = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(&app, Admin).await;
    let division = create_division("Division", &client, &app).await;
    let location = create_location("Location", &client, &app).await;
    let url = format!("{}/employees", app.address);
    do_post(
        &client,
        url.clone(),
        employee_body("Ann", "ann@derbe.test", &division, &location),
        "post",
    )
    .await;

    sqlx::query("UPDATE employees SET deleted_at = now() WHERE email = 'ann@derbe.test'")
        .execute(&app.ctx.pool)
        .await
        .unwrap();
    let rehire = do_post(
        &client,
        url,
        employee_body("Ann", "ann@derbe.test", &division, &location),
        "post",
    )
    .await;

    assert_eq!(rehire.status(), StatusCode::CREATED);

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn an_admin_can_create_a_manager() {
    let app = test_helpers::spawn_app().await;
    let places = places(&app).await;
    let admin = seed_employee(&app, "Adam", Admin, None).await;
    let client = login_client(&app, &admin).await;

    let response = create_as(
        &client,
        &app,
        &places,
        "mgr@derbe.test",
        Some("Manager"),
        None,
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = get_response_body_as_json(response).await;
    assert_eq!(body["employee"]["role"], "Manager");

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn an_admin_cannot_create_an_admin() {
    let app = test_helpers::spawn_app().await;
    let places = places(&app).await;
    let admin = seed_employee(&app, "Adam", Admin, None).await;
    let client = login_client(&app, &admin).await;

    let response = create_as(
        &client,
        &app,
        &places,
        "boss@derbe.test",
        Some("Admin"),
        None,
    )
    .await;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn omitting_the_role_creates_a_regular_employee() {
    let app = test_helpers::spawn_app().await;
    let places = places(&app).await;
    let admin = seed_employee(&app, "Adam", Admin, None).await;
    let client = login_client(&app, &admin).await;

    let response = create_as(&client, &app, &places, "new@derbe.test", None, None).await;

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = get_response_body_as_json(response).await;
    assert_eq!(body["employee"]["role"], "RegularUser");

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn a_manager_can_create_an_employee_who_reports_to_them() {
    let app = test_helpers::spawn_app().await;
    let places = places(&app).await;
    let mandy = seed_employee(&app, "Mandy", Manager, None).await;
    let client = login_client(&app, &mandy).await;

    let response = create_as(
        &client,
        &app,
        &places,
        "report@derbe.test",
        Some("RegularUser"),
        Some(mandy.unid),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = get_response_body_as_json(response).await;
    assert_eq!(body["employee"]["role"], "RegularUser");
    assert_eq!(body["employee"]["manager_unid"], mandy.unid.to_string());

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn a_manager_cannot_create_a_manager() {
    let app = test_helpers::spawn_app().await;
    let places = places(&app).await;
    let mandy = seed_employee(&app, "Mandy", Manager, None).await;
    let client = login_client(&app, &mandy).await;

    let response = create_as(
        &client,
        &app,
        &places,
        "peer@derbe.test",
        Some("Manager"),
        Some(mandy.unid),
    )
    .await;

    assert_eq!(response.status(), StatusCode::FORBIDDEN);

    cleanup_test_app(app).await;
}

#[tokio::test]
async fn a_manager_cannot_create_someone_who_reports_elsewhere() {
    let app = test_helpers::spawn_app().await;
    let places = places(&app).await;
    let mandy = seed_employee(&app, "Mandy", Manager, None).await;
    let otto = seed_employee(&app, "Otto", Manager, None).await;
    let client = login_client(&app, &mandy).await;

    let for_otto = create_as(
        &client,
        &app,
        &places,
        "ottos@derbe.test",
        Some("RegularUser"),
        Some(otto.unid),
    )
    .await;
    let for_nobody = create_as(
        &client,
        &app,
        &places,
        "nobodys@derbe.test",
        Some("RegularUser"),
        None,
    )
    .await;

    assert_eq!(for_otto.status(), StatusCode::FORBIDDEN);
    assert_eq!(for_nobody.status(), StatusCode::FORBIDDEN);

    cleanup_test_app(app).await;
}
