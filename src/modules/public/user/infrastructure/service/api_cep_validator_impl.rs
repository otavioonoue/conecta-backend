use std::{env, error::Error};

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::modules::public::user::{application::{dto::create_address_dto::CreateAddressDto, service::api_cep_validator::ApiCepValidator}, domain::entity::address::Address};

pub struct ApiCepValidatorImpl<T> {
    pub client: T,
    pub base_url: String,
    pub format: String
}

#[derive(Serialize, Deserialize)]
pub struct ApiCepValidatorResponse {
    cep: String,
    logradouro: String,
    complemento: String,
    unidade: String,
    bairro: String,
    localidade: String,
    uf: String,
    estado: String,
    regiao: String,
    ibge: String,
    gia: String,
    ddd: String,
    siafi: String
}

impl ApiCepValidatorImpl<Client> {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        let base_url = env::var("API_CEP_VALIDATOR").expect("Coudn't find API_CEP_VALIDATOR");
        let format = env::var("API_CEP_VALIDATOR_FORMAT").expect("Coudn't find API_CEP_VALIDATOR_FORMAT");
        ApiCepValidatorImpl { 
            client,
            base_url,
            format
        }
    }
}

impl ApiCepValidator for ApiCepValidatorImpl<Client> {
    async fn validate(&self, address: &CreateAddressDto) -> Result<Address, Box<dyn Error>> {
        let url = format!("{}{}{}", &self.base_url, &address.cep, &self.format);
        let resp: ApiCepValidatorResponse = self.client.get(url)
            .send()
            .await?
            .json()
            .await?;
        
        let address_domain = Address {
            id: String::from(""),
            cep: resp.cep,
            number: address.number.clone(),
            street: resp.logradouro,
            neighborhood: resp.bairro,
            city: resp.localidade,
            state: resp.estado,
            user_id: String::from("")
        };
        
        Ok(address_domain)
    }
}