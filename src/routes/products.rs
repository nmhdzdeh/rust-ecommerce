use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get},
};
use std::sync::{Arc, Mutex};

use crate::models::product::{Product, ProductInput};

type ProductList = Arc<Mutex<Vec<Product>>>;

pub fn product_routes() -> Router {
    let products = Arc::new(Mutex::new(vec![])); // shared state
    Router::new()
        .route("/products", get(get_products).post(create_product))
        .route("/products/:id", delete(delete_product) .put(update_product))
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

async fn delete_product(State(products): State<ProductList>, Path(id): Path<u32>) -> StatusCode {
    let mut data = products.lock().unwrap();
    let before_len = data.len();

    data.retain(|p| p.id != id);

    if data.len() < before_len {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn update_product(
    State(products): State<ProductList>,
    Path(id): Path<u32>,
    Json(payload): Json<ProductInput>,
) -> Result<Json<Product>, StatusCode> {
    let mut data = products.lock().unwrap();

    if let Some(product) = data.iter_mut().find(|p| p.id == id) {
        product.name = payload.name.clone();
        product.price = payload.price;

        Ok(Json(product.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
