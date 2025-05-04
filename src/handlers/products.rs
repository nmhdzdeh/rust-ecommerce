use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::models::product::Product;

pub async fn list_products(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Product>>, (axum::http::StatusCode, String)> {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(&pool)
        .await
        .map_err(|err| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(products))
}