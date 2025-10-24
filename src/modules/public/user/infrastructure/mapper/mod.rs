use std::str::FromStr;

use chrono::DateTime;
use sqlx::types::Uuid;

use crate::{modules::public::user::domain::entity::user::User, shared::infra::database::model::user_model::UserModel};

pub struct InfrastructureMapper;

impl InfrastructureMapper {
  pub fn to_data(user: User) -> UserModel {
    let address: Option<Uuid> = match user.address_id {
      Some(v) => Some(Uuid::from_str(&v).unwrap()),
      None => None
    };
    
    UserModel {
      id: Uuid::from_str(&user.id).unwrap_or_default(),
      name: user.name,
      email: user.email,
      phone: user.phone,
      cpf: user.cpf,
      active: user.active,
      address_id: address,
      password: user.password,
      created_at: DateTime::from_timestamp(user.created_at, 0).unwrap().naive_utc()
    }
  }
  
  pub fn to_domain(user_data: UserModel) -> User {
      User {
        id: user_data.id.to_string(),
        name: user_data.name,
        email: user_data.email,
        phone: user_data.phone,
        cpf: user_data.cpf,
        active: user_data.active,
        address_id: user_data.address_id.map(|v| v.to_string()),
        password: user_data.password,
        created_at: user_data.created_at.and_utc().timestamp()
      }
  }
}