mod web;

use axum::{routing::get, Router, Json};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use web::{
    routes::todo_routes,
    app_state::AppState,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");
    
    let db_url = std::env::var("DATABASE_URL").expect("check database url");
    let pool = PgPoolOptions::new().connect(&db_url).await.expect("check db connection string");
    
    let app = Router::new()
        .route("/health", get(health_check))
        .merge(todo_routes())
        .with_state(AppState::new(pool));

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<serde_json::Value> {
    const MESSAGE: &str = "API Services";
    
    let json_response = json!({
        "status": "ok",
        "message": MESSAGE,
    });
    
    Json(json_response)
}
