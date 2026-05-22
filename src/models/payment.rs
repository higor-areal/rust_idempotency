use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Payment{
    client: String,
    amount: f64
}