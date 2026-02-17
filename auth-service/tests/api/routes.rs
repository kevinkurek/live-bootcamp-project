use crate::helpers::{TestApp, get_random_email};

// Tokio's test macro is used to test async env
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;
    let response = app.get_root().await;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

// Implement tests for all other routes
// signup, 
// login, 
// logout, 
// verify-2fa,
// verify-token
// For now, simply assert that each route returns a 200 HTTP status code.

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

#[tokio::test]
async fn login_should_return_200() {
    let app = TestApp::new().await;
    let response = app.post_login().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_should_return_200() {
    let app = TestApp::new().await;
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa_should_return_200() {
    let app = TestApp::new().await;
    let response = app.post_verify_2fa().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_should_return_200() {
    let app = TestApp::new().await;
    let response = app.post_verify_token().await;
    assert_eq!(response.status().as_u16(), 200);
}