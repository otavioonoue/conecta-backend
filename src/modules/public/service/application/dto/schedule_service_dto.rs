use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Clone, Validate)]
pub struct ScheduleServiceDto {
    pub service_id: String,
    
    #[validate(length(max = 500))]
    pub description: Option<String>,
    
    pub address_id: String,
    
    pub schedule_to: i64
}