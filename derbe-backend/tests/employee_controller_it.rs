use reqwest::StatusCode;
use test_helpers::{
    cleanup_test_app, create_division, create_employee, create_location, do_post,
    get_response_body_as_json,
};

mod test_helpers;

#[tokio::test]
async fn get_employees() {
    let (app, config) = test_helpers::spawn_app().await;
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

    test_helpers::cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn get_employee() {
    let (app, config) = test_helpers::spawn_app().await;
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

    // Clean up the test application
    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn cannot_get_non_existant_employee() {
    let (app, config) = test_helpers::spawn_app().await;
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
    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn create_employee_success() {
    let (app, config) = test_helpers::spawn_app().await;
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
    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn cannot_create_employee_with_blank_first_name() {
    let (app, config) = test_helpers::spawn_app().await;
    let (_admin, client) = test_helpers::login_as(
        &app,
        derbe_backend::features::employees::model::EmployeeRole::Admin,
    )
    .await;

    let division = create_division("Division", &client, &app).await;
    let location = create_location("Location", &client, &app).await;

    let body = serde_json::json!({
        "first_name": "", // first name is blank
        "last_name": "Testerson",
        "job_title": "Tester",
        "division_id": division["id"],
        "employment_type": "Permanent",
        "location_id": location["id"],
        "manager_unid": None::<String>,
        "email": "test@derbe.com"
    });

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
    cleanup_test_app(&config.database, app).await;
}
