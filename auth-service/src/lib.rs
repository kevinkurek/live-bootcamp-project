use std::error::Error;

use axum::{
    Json, Router, http::{Method, StatusCode}, response::IntoResponse, routing::post
};
use redis::{Client, RedisResult};
use serde::{Serialize, Deserialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use utils::tracing::{make_span_with_request_id, on_request, on_response};

pub mod routes;
use routes::{signup, login, logout, verify_2fa, verify_token};

pub mod domain;
pub mod services;
pub mod app_state;
pub mod utils;
use domain::AuthAPIError;
use app_state::AppState;
use utils::constants::DROPLET_ORIGINS;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: axum::serve::Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {

        // Allow the app service(running on our local machine and in production) to call the auth service
        // let allowed_origins = build_allowed_origins();

        let cors = CorsLayer::new()
            // Allow GET and POST requests
            .allow_methods([Method::GET, Method::POST])
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(DROPLET_ORIGINS.clone());

        // Move the Router definition from `main.rs` to here.
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors)
            .layer(
                // Add a TraceLayer for HTTP requests to enable detailed tracing
                // This layer will create spans for each request using the make_span_with_request_id function,
                // and log events at the start and end of each request using on_request and on_response functions.
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok( Self {server, address})
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    // Create a new PostgreSQL connection pool
    PgPoolOptions::new().max_connections(5).connect(url).await
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}