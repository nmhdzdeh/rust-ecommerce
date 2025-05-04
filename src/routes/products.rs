use axum::{
    Json, Router,
    extract::State,
    routing::get,
};
use std::sync::{Arc, Mutex};

use crate::models::product::{Product, ProductInput};

type ProductList = Arc<Mutex<Vec<Product>>>;

pub fn product_routes() -> Router {
    let products = Arc::new(Mutex::new(vec![])); // shared state
    Router::new()
        .route("/products", get(get_products).post(create_product))
        .with_state(products)
}

async fn get_products(State(products): State<ProductList>) -> Json<Vec<Product>> {
    let data = products.lock().unwrap();
    Json(data.clone())
}

async fn create_product(
    State(products): State<ProductList>,
    Json(payload): Json<ProductInput>,
) -> Json<Product> {
    let mut data = products.lock().unwrap();
    let new_id = (data.len() + 1) as u32;

    let new_product = Product {
        id: new_id,
        name: payload.name,
        price: payload.price,
    };

    data.push(new_product.clone());
    Json(new_product)
}
