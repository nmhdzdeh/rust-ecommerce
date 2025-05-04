use serde::{Serialize,Deserialize};
use sqlx::FromRow;

#[derive(Clone)]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub in_stock: bool,
}