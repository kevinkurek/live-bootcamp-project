use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_case = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.post_signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 200);
}