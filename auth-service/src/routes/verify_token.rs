use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::{app_state::AppState, domain::AuthAPIError, utils::auth::validate_token};

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>
) -> impl IntoResponse {
    
    // validate token from auth.rs
    match validate_token(&request.token, state.banned_token_store.clone()).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthAPIError::InvalidToken)
    }
}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}