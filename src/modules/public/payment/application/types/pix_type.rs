use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pix {
    pub id: String,
    
    pub encoded_image: String,
    
    pub payload: String,
    
    pub allows_multiple_payments: bool,
    
    pub expiration_date: Option<String>,
    
    pub description: String,
}


impl Pix {
    pub fn new(id: String, encoded_image: String, payload: String, allows_multiple_payments: bool, expiration_date: Option<String>, description: String) -> Self {
        Pix { id, encoded_image, payload, allows_multiple_payments, expiration_date, description }
    }
}