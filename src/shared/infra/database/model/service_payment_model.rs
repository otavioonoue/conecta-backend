use std::str::FromStr;

use chrono::{DateTime, Utc};
use http::StatusCode;
use rust_decimal::Decimal;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{modules::public::payment::domain::entity::service_payment::PaymentKind, shared::infra::error::AppError};

#[derive(FromRow)]
pub struct ServicePaymentModel {
    pub id: Uuid,
    pub schedule_service_information_id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_payment_id: String,
    pub kind: PaymentKindModel,
    pub status: String,
    pub cost: Decimal,
    pub created_at: DateTime<Utc>
}

#[derive(sqlx::Type, Debug, Clone, PartialEq)]
#[sqlx(type_name = "payment_kind", rename_all = "UPPERCASE")]
pub enum PaymentKindModel {
    Budget,
    Scheduled,
}

impl From<PaymentKindModel> for PaymentKind {
    fn from(value: PaymentKindModel) -> Self {
        match value {
            PaymentKindModel::Budget => Self::Budget,
            PaymentKindModel::Scheduled => Self::Scheduled
        }
    }
}

impl FromStr for PaymentKindModel {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        match s {
            "BUDGET" => Ok(Self::Budget),
            "SCHEDULED" => Ok(Self::Scheduled),
            _ => Err(AppError::new(StatusCode::NOT_ACCEPTABLE, "PaymentKind not acceptable"))
        }
    }
}