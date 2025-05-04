use serde::Serialize;

#[derive(Serialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
}