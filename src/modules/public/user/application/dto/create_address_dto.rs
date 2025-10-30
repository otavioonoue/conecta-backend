use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateAddressDto {
    pub cep: String,
    pub number: String
}