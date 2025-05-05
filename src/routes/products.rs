use axum::{Router, routing::get};
use crate::handlers::products::{list_products, create_product};
use sqlx::PgPool;

pub fn create_product_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/products", get(list_products).post(create_product))
        .with_state(pool)
}