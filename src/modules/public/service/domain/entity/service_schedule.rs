use serde::Serialize;

#[derive(Serialize)]
pub struct ServiceSchedule {
    pub id: String,
    pub service_information_id: String,
    pub service_status_id: i32,
    pub description: String,
    pub scheduled_to: i64,
    pub scheduled_at: i64
}

impl ServiceSchedule {
    pub fn new(
        service_information_id: String,
        service_status_id: i32,
        description: String,
        scheduled_to: i64
    ) -> Self {
        ServiceSchedule { 
            id: String::from(""), 
            service_information_id, 
            service_status_id, 
            description,
            scheduled_to, 
            scheduled_at: 0
        }
    }
}