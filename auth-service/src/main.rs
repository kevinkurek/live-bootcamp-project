use std::{env, sync::Arc};
use sqlx::PgPool;
use tokio::sync::RwLock;
use auth_service::{Application, 
    app_state::AppState, get_postgres_pool, get_redis_client, 
    services::{MockEmailClient, 
        data_stores::{PostgresUserStore, RedisBannedTokenStore, RedisTwoFACodeStore}}, 
        utils::{constants::{DATABASE_URL, REDIS_HOST_NAME, prod}, tracing::init_tracing}
};

#[tokio::main]
async fn main() {

    init_tracing();

    // We will use this PostgreSQL pool in the next task! 
    let pg_pool = configure_postgresql().await;
    let redis_connection = Arc::new(RwLock::new(configure_redis()));

    // Local dev HashMap & HashSet stores
    // let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    // let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    // let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));

    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(redis_connection.clone())));
    let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));
    let mock_email_client = Arc::new(MockEmailClient);
    let app_state = AppState::new(user_store, 
        banned_token_store, 
        two_fa_code_store,
        mock_email_client,
    );

    let app_address = env::var("APP_ADDRESS").unwrap_or_else(|_| prod::APP_ADDRESS.to_string());

    let app = Application::build(app_state, &app_address)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database! 
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}
