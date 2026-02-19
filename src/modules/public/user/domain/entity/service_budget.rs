use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ServiceBudget {
    pub id: String,
    pub service_information_id: String,
    pub service_cost: i64,
    pub travel_cost: i64,
    pub description: String,
    pub service_budget_status_id: i16,
    pub created_at: i64
}

impl ServiceBudget {
    pub fn new(
        id: String,
        service_information_id: String,
        service_cost: i64,
        travel_cost: i64,
        description: String,
        service_budget_status_id: i16,
        created_at: i64
    ) -> Self {
        ServiceBudget { 
            id, 
            service_information_id, 
            service_cost, 
            travel_cost,
            description,
            service_budget_status_id,
            created_at
        }
    }
}