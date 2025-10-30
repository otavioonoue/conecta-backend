use crate::modules::public::user::{application::dto::create_user_dto::CreateUserDto, domain::entity::user::User};

pub struct ApplicationMapper;

impl ApplicationMapper {
  pub fn to_domain_user(dto: CreateUserDto) -> User {
    let user = User {
      id: String::from(""),
      name: dto.name,
      email: dto.email,
      phone: dto.phone,
      cpf: dto.cpf,
      active: false,
      password: dto.password,
      created_at: 0
    };
    
    return user;
  }
  
  // pub fn to_domain_address(dto: CreateAddressDto, other_fields: CreateAddressFieldsDto) -> Address {
  //     let address = Address {
  //         id: String::from(""),
  //         cep: dto.cep,
  //         number: dto.number,
  //         street: other_fields.street,
  //         neighborhood: other_fields.neighborhood,
  //         city: other_fields.city,
  //         state: other_fields.state,
  //         created_at: 0,
  //         user_id: String::from("")
  //     };
      
  //     return address;
  // }
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