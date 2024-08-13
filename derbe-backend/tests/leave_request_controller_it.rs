use reqwest::StatusCode;

mod test_helpers;

#[tokio::test]
async fn create_leave_request_fails_with_unknown_employee() {
    let (app, config) = test_helpers::spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "employee_id": 0,
        "leave_type_id": 1,
        "leave_status": "Pending",
        "values": serde_json::json!({
            "date_from": "2024-05-20",
            "date_to": "2024-05-24",
            "first_day_half": false,
            "last_day_half": false,
            "total_duration": 5.0,
        }),
        "date_requested": "2024-05-11T13:48:48.739+00:00",
        "date_finalised": null,
        "notes": null
    });

    let response = client
        .post(format!("{}/leaveRequests", app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to create leave request");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Print the response headers
    println!("Response headers: {:#?}", response.headers());

    // Print the raw response text
    let response_text = response.text().await.expect("Failed to get response text");
    println!("Response text: {}", response_text);

    // Attempt to parse the response as JSON
    let error_message: serde_json::Value =
        serde_json::from_str(&response_text).expect("Failed to parse JSON");
    assert_eq!(error_message["error"], "Employee with id 0 not found!");

    test_helpers::cleanup_test_app(&config.database, app).await;
}
