use reqwest::StatusCode;
use test_helpers::{cleanup_test_app, create_division, get_response_body_as_json, spawn_app};

mod test_helpers;

#[tokio::test]
async fn create_division_success() {
    let (app, config) = spawn_app().await;
    let client = reqwest::Client::new();
    let name = "The Division";

    let response = create_division(name, &client, &app).await;

    // Assert that the response has an "id" field and it's a positive number
    assert!(
        response["id"].as_i64().unwrap() > 0,
        "Division ID should be greater than 0"
    );

    // Assert that the "name" field matches the input
    assert_eq!(response["name"], name);

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn cannot_create_divisions_with_duplicate_names() {
    let (app, config) = spawn_app().await;
    let client = reqwest::Client::new();

    let name = "The Division";

    create_division(name, &client, &app).await;

    let body = serde_json::json!({
        "name": "The Division"
    });

    let response = client
        .post(format!("{}/divisions", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to create division");

    assert_eq!(response.status(), StatusCode::CONFLICT);

    let response_text: serde_json::Value = get_response_body_as_json(response).await;

    assert_eq!(
        response_text["error"],
        "Division with this name already exists!"
    );

    // Clean up the test application
    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn get_division() {
    let (app, config) = spawn_app().await;
    let client = reqwest::Client::new();

    let name = "The Division";

    let response: serde_json::Value = create_division(name, &client, &app).await;
    let id = &response["id"];

    let response = client
        .get(format!("{}/divisions/{}", app.address, id))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(response.status(), StatusCode::OK);
    let response_text: serde_json::Value = get_response_body_as_json(response).await;
    assert_eq!(response_text["id"].to_string(), id.to_string());
    assert_eq!(response_text["name"], name);

    // Clean up the test application
    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn get_divisions() {
    let (app, config) = spawn_app().await;
    let client = reqwest::Client::new();

    let name = "The Division";

    create_division(name, &client, &app).await;

    let response = client
        .get(format!("{}/divisions", app.address))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(response.status(), StatusCode::OK);

    let response_text: serde_json::Value = get_response_body_as_json(response).await;

    assert!(response_text.is_array());
    let divisions = response_text.as_array().unwrap();

    assert_eq!(divisions.len(), 1, "Expected one division in the response");

    assert!(
        divisions[0]["id"].as_i64().unwrap_or(0) > 0,
        "Division ID should be greater than 0"
    );

    assert_eq!(divisions[0]["name"], name);

    // Clean up the test application
    cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn cannot_get_nonexistant_division() {
    let (app, config) = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/divisions/{}", app.address, 0))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let response_text: serde_json::Value = get_response_body_as_json(response).await;
    assert_eq!(response_text["error"], "Division with id 0 not found!");

    // Clean up the test application
    cleanup_test_app(&config.database, app).await;
}
