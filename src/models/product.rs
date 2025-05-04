use serde::{Serialize,Deserialize};

#[derive(Serialize)]
#[derive(Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
}

#[derive(Deserialize)]
pub struct ProductInput {
    pub name: String,
    pub price: f64,
}