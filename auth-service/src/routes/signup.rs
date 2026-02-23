use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState, 
    domain::{AuthAPIError, User}
};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>) -> Result<impl IntoResponse, AuthAPIError> {

    let email = request.email;
    let password = request.password;

    // return early if:
    // - email is empty or does not contain '@'
    // - password is less than 8 characters
    if email.is_empty() || !email.contains("@") || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // define a user from incoming request data
    let user = User::new(email, password, request.requires_2fa);

    // request to mutate user_store
    let mut user_store = state.user_store.write().await;

    // check if user already exists
    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    if user_store.add_user(user).await.is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }

    // Check response
    let response = Json( SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct  SignupResponse {
    pub message: String,
}