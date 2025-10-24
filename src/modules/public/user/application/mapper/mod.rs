use crate::modules::public::user::{application::dto::create_user_dto::CreateUserDto, domain::entity::user::User};

pub struct ApplicationMapper;

impl ApplicationMapper {
  pub fn to_domain(dto: CreateUserDto) -> User {
    let user = User {
      id: String::from(""),
      name: dto.name,
      email: dto.email,
      phone: dto.phone,
      cpf: dto.cpf,
      active: false,
      address_id: None,
      password: dto.password,
      created_at: 0
    };
    
    return user;
  }
  // pub fn toData(user: User) -> UserModel {
  //   let address: Option<Uuid> = match user.address_id {
  //     Some(v) => Some(Uuid::from_str(&v).unwrap()),
  //     None => None
  //   };
    
  //   UserModel {
  //     id: user.id,
  //     name: user.name,
  //     email: user.email,
  //     phone: user.phone,
  //     cpf: user.cpf,
  //     active: user.active,
  //     address_id: address,
  //     password: user.password,
  //     created_at: None
  //   }
  // } 
}