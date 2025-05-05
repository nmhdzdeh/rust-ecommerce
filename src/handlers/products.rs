use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::models::product::{NewProduct, Product};

pub async fn list_products(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Product>>, (axum::http::StatusCode, String)> {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                err.to_string(),
            )
        })?;

    Ok(Json(products))
}

pub async fn create_product(
    State(pool): State<PgPool>,
    Json(payload): Json<NewProduct>,
) -> Result<Json<Product>, (axum::http::StatusCode, String)> {
    let row = sqlx::query_as::<_, Product>(
        "INSERT INTO products (name, description, price, in_stock)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.price)
    .bind(payload.in_stock)
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        )
    })?;

    Ok(Json(row))
}
