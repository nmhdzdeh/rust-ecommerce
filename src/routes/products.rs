use axum::{routing::get, Json, Router};
use crate::models::product::Product;

pub fn product_routes() -> Router {
    Router::new().route("/products", get(get_products))
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