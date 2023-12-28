use dotenv::dotenv;
use std::error::Error;

use sqlx::postgres::PgPool;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use userservice::v1::routes::{user_create, user_delete, user_read, user_update, users_read};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load environment variables
    dotenv().ok();

    let api_port = std::env::var("APIPORT").expect("APIPORT must be set.");
    let db_name = std::env::var("DBNAME").expect("DBNAME must be set.");
    let db_user = std::env::var("DBUSER").expect("DBUSER must be set.");
    let db_password = std::env::var("DBPASSWORD").expect("DBPASSWORD must be set.");
    let db_host = std::env::var("DBHOST").expect("DBHOST must be set.");
    let db_port = std::env::var("DBPORT").expect("DBPORT must be set.");

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    let pool = PgPool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let router = Router::new()
        .route("/v1/user", get(users_read))
        .route("/v1/user", post(user_create))
        .route("/v1/user/:id", get(user_read))
        .route("/v1/user/:id", patch(user_update))
        .route("/v1/user/:id", delete(user_delete))
        .with_state(pool);

    let api_url = format!("0.0.0.0:{}", api_port);
    println!("API Running on {}", api_port);
    let listener = tokio::net::TcpListener::bind(api_url).await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
