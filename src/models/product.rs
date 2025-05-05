use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub in_stock: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub in_stock: bool,
}
