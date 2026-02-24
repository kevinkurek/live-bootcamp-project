use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState, 
    domain::{AuthAPIError, User, Email, Password}
};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>) -> Result<impl IntoResponse, AuthAPIError> {

    // "Parse, don't validate" - Email & Password structs parse and validate validity of values so 
    // we don't have to write validation logic here.
    let email = Email::parse(request.email.clone())
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password.clone())
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

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