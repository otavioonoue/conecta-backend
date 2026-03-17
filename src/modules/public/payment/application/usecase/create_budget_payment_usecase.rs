use async_trait::async_trait;
use http::StatusCode;
use rust_decimal::Decimal;

use crate::{modules::public::{auth::infrastructure::jwt::claim::Claims, payment::{application::{dto::{create_budget_payment::CreateBudgetPaymentDto, payment_response::PaymentResponse}, usecase::usecase::UseCase}, appstate::PaymentAppState, domain::entity::{service_payment::{PaymentKind, ServicePayment}}}}, shared::infra::{error::AppError, helpers::currency::CurrencyHelper}};

pub struct CreateBudgetPaymentUseCase;

type Input = (Claims, CreateBudgetPaymentDto);
type Output = Result<PaymentResponse, AppError>;

#[async_trait]
impl UseCase<Input, Output> for CreateBudgetPaymentUseCase {
    async fn execute(&self, (_, input): Input, s: PaymentAppState) -> Output {
    
        let optional_service_information = s.service_repository.find_service_information_by_id(input.service_information_id).await?;
    
        let Some(service_information) = optional_service_information else {
            return Err(AppError::new(StatusCode::FORBIDDEN, "Service Information not found for your account"));
        };
        
        // if service_information.service_step_id != 7 {
        //     return Err(AppError::new(StatusCode::UNPROCESSABLE_ENTITY, "The service budget hasn't been confirmed yet"))
        // }
        // service_information.service_step_id = 9;
        
        let services_budgets = s.service_repository.find_budgets_approved_by_service_information_id(service_information.id.clone()).await?;
        
        if services_budgets.len() <= 0 {
            return Err(AppError::new(StatusCode::UNPROCESSABLE_ENTITY, "It's not possible to create payment without an approved budget"));
        }
    
        let optional_service = s.service_repository.find_service_by_id(service_information.service_id.clone()).await?;
        
        let Some(service) = optional_service else {
           	return Err(AppError::new(StatusCode::NOT_FOUND, "Service not found"));
        };
        
        let mut total_cost = Decimal::new(000, 2);
        
        for sb in services_budgets {
            total_cost = total_cost + s.payment_budget_service.calculate_total_cost(sb.travel_cost, sb.service_cost);
        }
        
        let payment_name = format!("Orçamento Serviço: {}", service.name);
    
        let payment_response = s
            .payment_service.create_payment(payment_name, input.method, total_cost)
            .await?;
        
        // For while in this POC...
        let service_payment = ServicePayment {
            id: String::from(""),
            schedule_service_information_id: service_information.id.clone(),
            user_id: service_information.user_id.clone(),
            provider: String::from("ASAAS"), 
            provider_payment_id: payment_response.payment_id.clone(),
            kind: PaymentKind::Budget,
            status: String::from("PENDING"),
            cost: CurrencyHelper::to_cents(total_cost),
            created_at: 0
        };
        
        s.payment_repository.create_service_payment(service_payment).await?;
        
        s.service_repository.update_service_information(service_information).await?;
        
        Ok(payment_response)
    }
}