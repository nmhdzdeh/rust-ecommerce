use axum::{routing::get, Router};

mod models;
mod routes;
mod db;


use routes::products::product_routes;
use db::init_db;

#[tokio::main]
async fn main() {

    let db_pool = init_db().await;
    
    let app = Router::new()
        .route("/", get(root))
        .with_state(db_pool)
        .merge(product_routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Rust E-Commerce Backend Connected to PostgreSQL!"
}