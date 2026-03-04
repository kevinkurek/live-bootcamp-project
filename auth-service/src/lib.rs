use std::error::Error;
use std::env;

use axum::{
    Json, Router, http::{Method, StatusCode}, response::IntoResponse, routing::post
};
use serde::{Serialize, Deserialize};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod routes;
use routes::{signup, login, logout, verify_2fa, verify_token};

pub mod domain;
pub mod services;
pub mod app_state;
pub mod utils;
use domain::AuthAPIError;
use app_state::AppState;

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
        let mut allowed_origins = vec!["http://localhost:8000".parse()?];
        if let Ok(droplet_ip) = env::var("DROPLET_IP") {
            allowed_origins.push(format!("http://{}:8000", droplet_ip).parse()?);
        }

        let cors = CorsLayer::new()
            // Allow GET and POST requests
            .allow_methods([Method::GET, Method::POST])
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        // Move the Router definition from `main.rs` to here.
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok( Self {server, address})
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
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








