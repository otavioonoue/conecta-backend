use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum PaymentMethodType {
    Pix,
    Credit,
    Debit
}