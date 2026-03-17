use dotenvy::{dotenv, from_filename};
use lazy_static::lazy_static;
use std::env as std_env;
use axum::http::HeaderValue;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref DATABASE_URL: String = set_db_url();
    // pub static ref REDIS_HOST_NAME: String = set_redis_host();
    pub static ref DROPLET_ORIGINS: Vec<HeaderValue> = build_allowed_origins();

}


fn set_token() -> String {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

fn set_db_url() -> String {
    dotenv().ok(); // Load environment variables
    let db_url = std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set.");
    if db_url.is_empty() {
        panic!("DATABASE_URL must not be empty.");
    }
    db_url
}

fn build_allowed_origins() -> Vec<HeaderValue> {
    // Try cwd, then workspace-root path
    dotenv().ok();
    from_filename("auth-service/.env").ok();

    let mut origins = vec![
        HeaderValue::from_str("http://localhost:8000").expect("valid localhost origin")
    ];
    if let Ok(ip) = std_env::var(env::DROPLET_IP_ENV_VAR) {
        if let Ok(hv) = HeaderValue::from_str(&format!("http://{}:8000", ip)) {
            origins.push(hv);
        }
    }
    origins
}

// This value determines how long the JWT auth token is valid for
pub const TOKEN_TTL_SECONDS: i64 = 600; // 10 minutes

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const DROPLET_IP_ENV_VAR: &str = "DROPLET_IP";
}

pub const JWT_COOKIE_NAME: &str = "jwt";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}
