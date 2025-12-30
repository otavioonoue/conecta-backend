use std::str::FromStr;

use chrono::DateTime;
use sqlx::types::Uuid;

use crate::{
    modules::public::user::domain::entity::{address::Address, user::User},
    shared::infra::database::model::{address_model::AddressModel, user_model::UserModel},
};

pub struct InfrastructureMapper;

impl InfrastructureMapper {
    pub fn to_data_user(user: User) -> UserModel {
        UserModel {
            id: Uuid::from_str(&user.id).unwrap_or_default(),
            name: user.name,
            email: user.email,
            phone: user.phone,
            cpf: user.cpf,
            active: user.active,
            password: user.password,
            created_at: DateTime::from_timestamp(user.created_at, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    pub fn to_domain_user(user_data: UserModel) -> User {
        User {
            id: user_data.id.to_string(),
            name: user_data.name,
            email: user_data.email,
            phone: user_data.phone,
            cpf: user_data.cpf,
            active: user_data.active,
            password: user_data.password,
            created_at: user_data.created_at.and_utc().timestamp(),
        }
    }

    pub fn to_data_address(address: Address, user_id: String) -> AddressModel {
        AddressModel {
            id: Uuid::from_str(&address.id).unwrap_or_default(),
            cep: address.cep,
            number: address.number,
            street: address.street,
            neighborhood: address.neighborhood,
            city: address.city,
            state: address.state,
            user_id: Uuid::from_str(&user_id).unwrap_or_default()
        }
    }
    
    pub fn to_domain_address(address_data: AddressModel) -> Address {
        Address {
            id: address_data.id.to_string(), 
            cep: address_data.cep, 
            number: address_data.number, 
            street: address_data.street, 
            neighborhood: address_data.neighborhood, 
            city: address_data.city, 
            state: address_data.state,
            user_id: address_data.user_id.to_string()
        }
    }
}
