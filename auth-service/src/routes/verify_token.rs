use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::{domain::AuthAPIError, utils::auth::validate_token};

pub async fn verify_token(
    Json(request): Json<VerifyTokenRequest>
) -> impl IntoResponse {
    
    // validate token from auth.rs
    match validate_token(&request.token).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthAPIError::InvalidToken)
    }
}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}