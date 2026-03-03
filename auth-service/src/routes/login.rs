use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password}};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>) -> Result<impl IntoResponse, AuthAPIError> {

    let email = Email::parse(request.email)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    // check if user is in user_store
    let user_store = &state.user_store.read().await;

    // validate if email and password are correct
    if user_store.validate_user(&email, &password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    // check if email exists
    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return Err(AuthAPIError::IncorrectCredentials),
    };

    Ok(StatusCode::OK)
}