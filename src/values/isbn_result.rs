use serde::Serialize;

use super::isbn::ISBN;

#[derive(Debug, Serialize)]
pub struct ISBNResult {
    isbn: u64,
    name: String,
    price: f64
}

impl ISBNResult {
    pub fn new(isbn: ISBN, name: String, price: f64) -> Self {
        ISBNResult {
            isbn: isbn.value(),
            name,
            price
        }
    }
}
