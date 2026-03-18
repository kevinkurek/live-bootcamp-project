use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::RwLock;
use auth_service::{Application, 
    app_state::AppState, 
    get_postgres_pool, 
    services::MockEmailClient,
    services::data_stores::{HashmapTwoFACodeStore, PostgresUserStore, HashsetBannedTokenStore},  
    utils::constants::{DATABASE_URL, prod}
};

#[tokio::main]
async fn main() {

    // We will use this PostgreSQL pool in the next task! 
    let pg_pool = configure_postgresql().await;

    // let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let mock_email_client = Arc::new(MockEmailClient);
    let app_state = AppState::new(user_store, 
        banned_token_store, 
        two_fa_code_store,
        mock_email_client,
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
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