use rust_decimal::{Decimal};

pub struct PaymentBudgetServiceImpl;

pub trait PaymentBudgetService: Send + Sync {
    fn calculate_total_cost(&self, travel_cost: i64, service_cost: i64) -> Decimal;
}

impl PaymentBudgetService for PaymentBudgetServiceImpl {
    fn calculate_total_cost(&self, travel_cost: i64, service_cost: i64) -> Decimal {
        let service_cost = Decimal::new(service_cost, 2);
        let travel_cost = Decimal::new(travel_cost, 2);
        
        let total_cost = service_cost - travel_cost;
        
        if !total_cost.is_sign_negative() {
            return total_cost;
        }
        
        return Decimal::new(0, 2);
    }
}