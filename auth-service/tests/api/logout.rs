use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;

use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    // sign someone up
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password1234",
        "requires2FA": false
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // sign someone in
    let signin_body = serde_json::json!({
        "email": random_email,
        "password": "password1234",
    });
    let response = app.post_login(&signin_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // get auth cookie, confirm it's not empty
    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    // log them out
    let _token = auth_cookie.value();
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(auth_cookie.value().is_empty());

}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(auth_cookie.value().is_empty());

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Missing auth token".to_owned()
    );
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    let response = app.post_logout().await;

    assert_eq!(
        response.status().as_u16(),
        400,
        "Teh API did not return a 400 BAD REQUEST",
    );

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    assert!(auth_cookie.is_none());

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
            "Missing auth token".to_owned()
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 401);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    assert!(auth_cookie.is_none());

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid auth token".to_owned()
    );
}