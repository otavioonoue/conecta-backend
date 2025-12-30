use crate::modules::public::consultant::{application::dto::create_consultant_dto::CreateConsultantDto, domain::entity::consultant::Consultant};

pub struct ApplicationMapper;

impl ApplicationMapper {
	pub fn to_domain_consultant(dto: CreateConsultantDto) -> Consultant {
		let consultant = Consultant {
			id: String::from(""),
			name: dto.name,
			email: dto.email,
			phone: dto.phone,
			password: dto.password,
			active: false,
			created_at: 0
		};
		
		return consultant;
	}
}