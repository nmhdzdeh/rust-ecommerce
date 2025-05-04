use axum::{Json, Router, routing::get};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/products", get(get_products));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to Rust E-Commerce Backend!"
}

#[derive(Serialize)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

async fn get_products() -> Json<Vec<Product>> {
    let products = vec![
        Product {
            id: 1,
            name: "Shoes".to_string(),
            price: 79.99,
        },
        Product {
            id: 2,
            name: "Bag".to_string(),
            price: 45.50,
        },
    ];

    Json(products)
}
