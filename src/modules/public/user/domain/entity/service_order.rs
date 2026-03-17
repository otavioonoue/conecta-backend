use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ServiceOrder {
    pub id: String,
    pub service_information_id: String,
    pub final_cost: i64,
    pub description: String,
    pub service_order_status_id: i16,
    pub scheduled_to: i64,
    pub scheduled_at: i64
}

impl ServiceOrder {
    pub fn new(
        id: String,
        service_information_id: String,
        final_cost: i64,
        description: String,
        service_order_status_id: i16,
        scheduled_to: i64,
        scheduled_at: i64
    ) -> Self {
        ServiceOrder { 
            id, 
            service_information_id, 
            final_cost, 
            description,
            service_order_status_id,
            scheduled_to,
            scheduled_at
        }
    }
}