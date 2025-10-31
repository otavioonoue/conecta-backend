use std::error::Error;

use crate::modules::public::user::{application::dto::{create_address_dto::CreateAddressDto}, domain::entity::address::Address};

pub trait ApiCepValidator {
    fn validate(&self, address: &CreateAddressDto) -> impl Future<Output = Result<Address, Box<dyn Error>>>;
}