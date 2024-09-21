use reqwest::StatusCode;

mod test_helpers;

#[tokio::test]
async fn create_location() {
    let (app, config) = test_helpers::spawn_app().await;
    let client = reqwest::Client::new();
    let name = "The Location";

    let response = test_helpers::create_location(name, &client, &app).await;

    // Assert that the response has an "id" field and it's a positive number
    assert!(
        response["id"].as_i64().unwrap() > 0,
        "Location ID should be greater than 0"
    );

    // Assert that the "name" field matches the input
    assert_eq!(response["name"], name);

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn cannot_create_locations_with_duplicate_names() {
    let (app, config) = test_helpers::spawn_app().await;
    let client = reqwest::Client::new();

    let name = "The Location";

    test_helpers::create_location(name, &client, &app).await;

    let body = serde_json::json!({
        "name": "The Location"
    });

    let response = client
        .post(format!("{}/locations", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to create location");

    assert_eq!(response.status(), StatusCode::CONFLICT);

    let response_text: serde_json::Value = test_helpers::get_response_body_as_json(response).await;

    assert_eq!(
        response_text["error"],
        "Location with this name already exists!"
    );

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn get_location() {
    let (app, config) = test_helpers::spawn_app().await;
    let client = reqwest::Client::new();

    let name = "The Location";

    let response: serde_json::Value = test_helpers::create_location(name, &client, &app).await;
    let id = &response["id"];

    let response = client
        .get(format!("{}/locations/{}", app.address, id))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(response.status(), StatusCode::OK);
    let response_text: serde_json::Value = test_helpers::get_response_body_as_json(response).await;
    assert_eq!(response_text["id"].to_string(), id.to_string());
    assert_eq!(response_text["name"], name);

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn get_locations() {
    let (app, config) = test_helpers::spawn_app().await;
    let client = reqwest::Client::new();

    let name = "The Location";

    test_helpers::create_location(name, &client, &app).await;

    let response = client
        .get(format!("{}/locations", app.address))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(response.status(), StatusCode::OK);

    let response_text: serde_json::Value = test_helpers::get_response_body_as_json(response).await;

    assert!(response_text.is_array());
    let locations = response_text.as_array().unwrap();

    assert_eq!(locations.len(), 1, "Expected one location in the response");

    assert!(
        locations[0]["id"].as_i64().unwrap_or(0) > 0,
        "Location ID should be greater than 0"
    );

    assert_eq!(locations[0]["name"], name);

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}

#[tokio::test]
async fn cannot_get_nonexistant_location() {
    let (app, config) = test_helpers::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/locations/{}", app.address, 0))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let response_text: serde_json::Value = test_helpers::get_response_body_as_json(response).await;
    assert_eq!(response_text["error"], "Location with id 0 not found!");

    // Clean up the test application
    test_helpers::cleanup_test_app(&config.database, app).await;
}
