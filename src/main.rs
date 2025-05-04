use axum::{Router, routing::get};

mod db;
mod handlers;
mod models;
mod routes;

use routes::products::create_product_routes;


use db::init_db;

#[tokio::main]
async fn main() {
    let db_pool = init_db().await;

    let app = Router::new()
    .route("/", get(root))
    .merge(create_product_routes(db_pool.clone()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Rust E-Commerce Backend Connected to PostgreSQL!"
}
