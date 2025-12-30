use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ScheduledServiceWithAddress {
    pub scheduled_id: Uuid,      // ss.id
    pub name: String,            // s.name
    pub travel_cost: f64,        // s.travel_cost

    pub address_id: Uuid,        // a.id
    pub cep: String,             // a.cep
    pub number: i32,             // a.number
    pub street: String,          // a.street
    pub neighborhood: String,    // a.neighborhood
    pub city: String,            // a.city
    pub state: String,           // a.state

    pub user_id: Uuid           // a.user_id
}