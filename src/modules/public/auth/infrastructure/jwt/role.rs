use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum Role {
    USER,
    PROVIDER,
    ADMIN
}