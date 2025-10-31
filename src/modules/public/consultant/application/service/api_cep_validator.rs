use std::error::Error;

use crate::modules::public::user::{application::dto::{create_address_dto::CreateAddressDto}, domain::entity::address::Address};

pub trait ApiCepValidator {
    async fn validate(&self, address: &CreateAddressDto) -> Result<Address, Box<dyn Error>>;
}