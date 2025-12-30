use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum PaymentMethodType {
    Pix,
    Credit,
    Debit
}