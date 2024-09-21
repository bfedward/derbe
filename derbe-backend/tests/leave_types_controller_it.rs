use reqwest::StatusCode;
use test_helpers::{create_leave_type, get_response_body_as_json, spawn_app};

mod test_helpers;

#[tokio::test]
async fn create_leave_type_success() {
    let (app, config) = test_helpers::spawn_app().await;
    let client = reqwest::Client::new();
    let name = "Leave Type";

    let response = create_leave_type(name, &client, &app).await;

    // Assert that the response has an "id" field and it's a positive number
    assert!(
        response["id"].as_i64().unwrap() > 0,
        "Leave Type ID should be greater than 0"
    );

    // Assert that the "name" field matches the input
    assert_eq!(response["name"], name);

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn cannot_create_leave_types_with_duplicate_names() {
    let (app, config) = spawn_app().await;
    let client = reqwest::Client::new();

    let name = "Leave Type";

    create_leave_type(name, &client, &app).await;

    let body = serde_json::json!({
        "name": name,
        "default_quota": Some(10.0),
        "colour": "#FFFFFF",
        "can_book_in_future": true,
        "show_in_others_days": true,
        "default_in_others_days": true,
        "default_in_leave_request": true,
        "show_in_approve_days": true
    });

    let response = client
        .post(format!("{}/leaveTypes", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to create leave type");

    assert_eq!(response.status(), StatusCode::CONFLICT);

    let response_text: serde_json::Value = get_response_body_as_json(response).await;

    assert_eq!(
        response_text["error"],
        "Leave Type with this name already exists!"
    );

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn cannot_create_leave_types_with_negative_default_quota() {
    let (app, config) = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "name": "Leave Type",
        "default_quota": Some(-10.0),
        "colour": "#FFFFFF",
        "can_book_in_future": true,
        "show_in_others_days": true,
        "default_in_others_days": true,
        "default_in_leave_request": true,
        "show_in_approve_days": true
    });

    let response = client
        .post(format!("{}/leaveTypes", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to create leave type");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let response_text: String = test_helpers::get_response_body(response).await;

    assert!(response_text.contains("default_quota: value must be non-negative"));

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn get_leave_types_success() {
    let (app, config) = spawn_app().await;
    let client = reqwest::Client::new();

    let name = "Leave Type";

    create_leave_type(name, &client, &app).await;

    let response = client
        .get(format!("{}/leaveTypes", app.address))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(response.status(), StatusCode::OK);

    let response_text: serde_json::Value = get_response_body_as_json(response).await;

    assert!(response_text.is_array());
    let leave_types = response_text.as_array().unwrap();

    assert_eq!(
        leave_types.len(),
        1,
        "Expected one leave type in the response"
    );

    assert!(
        leave_types[0]["id"].as_i64().unwrap_or(0) > 0,
        "Leave Type ID should be greater than 0"
    );

    assert_eq!(leave_types[0]["name"], name);

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn cannot_get_nonexistant_leave_type() {
    let (app, config) = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/leaveTypes/{}", app.address, 0))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let response_text: serde_json::Value = get_response_body_as_json(response).await;
    assert_eq!(response_text["error"], "Leave Type with id 0 not found!");

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn get_leave_type() {
    let (app, config) = spawn_app().await;
    let client = reqwest::Client::new();

    let name = "Leave Type";

    let create_response: serde_json::Value = create_leave_type(name, &client, &app).await;
    let id = &create_response["id"];

    let response = client
        .get(format!("{}/leaveTypes/{}", app.address, id))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(response.status(), StatusCode::OK);
    let response_text: serde_json::Value = get_response_body_as_json(response).await;
    assert_eq!(response_text["id"].to_string(), id.to_string());
    assert_eq!(response_text["name"], name);
    assert_eq!(
        response_text["default_quota"],
        create_response["default_quota"]
    );
    assert_eq!(response_text["colour"], create_response["colour"]);
    assert_eq!(
        response_text["can_book_in_future"],
        create_response["can_book_in_future"]
    );
    assert_eq!(
        response_text["show_in_others_days"],
        create_response["show_in_others_days"]
    );
    assert_eq!(
        response_text["default_in_others_days"],
        create_response["default_in_others_days"]
    );
    assert_eq!(
        response_text["default_in_leave_request"],
        create_response["default_in_leave_request"]
    );
    assert_eq!(
        response_text["show_in_approve_days"],
        create_response["show_in_approve_days"]
    );

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}
