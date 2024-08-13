mod test_helpers;

#[tokio::test]
async fn get_employees() {
    let (app, config) = test_helpers::spawn_app().await;
    let client = reqwest::Client::new();

    println!("{}", &app.address);

    let response = client
        .get(format!("{}/employees", &app.address))
        .send()
        .await
        .expect("Failed to get employees");

    assert!(response.status().is_success());

    test_helpers::cleanup_test_app(&config.database, app).await;
}
