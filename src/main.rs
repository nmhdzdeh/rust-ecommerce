use axum::{routing::get, Router};

mod models;
mod routes;

use routes::products::product_routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .merge(product_routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to Rust E-Commerce Backend!"
}