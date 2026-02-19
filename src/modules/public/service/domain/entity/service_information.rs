use serde::Serialize;

#[derive(Serialize)]
pub struct ServiceInformation {
    pub id: String,
    pub user_id: String,
    pub service_id: String,
    pub consultant_id: Option<String>,
    pub service_step_id: i16,
    pub address_id: String,
    pub scheduled_at: i64
}

impl ServiceInformation {
    pub fn new(
        user_id: String,
        service_id: String,
        consultant_id: String,
        service_step_id: i16,
        address_id: String
    ) -> Self {
        ServiceInformation { 
            id: String::from(""), 
            user_id, 
            service_id, 
            consultant_id: Some(consultant_id),
            service_step_id,
            address_id,
            scheduled_at: 0
        }
    }
}