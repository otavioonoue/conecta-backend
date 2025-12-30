use serde::Serialize;

#[derive(Serialize)]
pub struct ServiceSchedule {
    pub id: String,
    pub service_id: String,
    pub user_id: String,
    pub consultant_id: String,
    pub service_status_id: i32,
    pub description: String,
    pub address_id: String,
    pub scheduled_to: i64,
    pub scheduled_at: i64
}

impl ServiceSchedule {
    pub fn new(
        service_id: String,
        user_id: String,
        consultant_id: String,
        service_status_id: i32,
        description: String,
        address_id: String,
        scheduled_to: i64
    ) -> Self {
        ServiceSchedule { 
            id: String::from(""), 
            service_id, 
            user_id, 
            consultant_id, 
            service_status_id, 
            description,
            address_id,
            scheduled_to, 
            scheduled_at: 0
        }
    }
}