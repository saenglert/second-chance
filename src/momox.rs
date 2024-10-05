use crate::values::ISBN;

pub fn get(isbn: ISBN) -> Result<f64, String> {
    
    Ok(isbn.value() as f64)
}

