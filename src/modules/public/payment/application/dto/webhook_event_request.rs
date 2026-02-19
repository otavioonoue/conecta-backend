use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookEventRequest {
    pub id: String,
    pub event: String,
    pub dateCreated: String,
    pub account: Account,
    pub payment: Payment,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: String,
    pub ownerId: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Payment {
    pub object: String,
    pub id: String,
    pub dateCreated: String,
    pub customer: String,
    pub checkoutSession: Option<String>,
    pub paymentLink: Option<String>,
    pub value: f64,
    pub netValue: f64,
    pub originalValue: Option<f64>,
    pub interestValue: Option<f64>,
    pub description: Option<String>,
    pub billingType: BillingType,
    pub pixTransaction: Option<String>,
    pub status: PaymentStatus,
    pub dueDate: String,
    pub originalDueDate: String,
    pub paymentDate: Option<String>,
    pub clientPaymentDate: Option<String>,
    pub installmentNumber: Option<u32>,
    pub invoiceUrl: Option<String>,
    pub invoiceNumber: Option<String>,
    pub externalReference: Option<String>,
    pub deleted: bool,
    pub anticipated: bool,
    pub anticipable: bool,
    pub creditDate: Option<String>,
    pub estimatedCreditDate: Option<String>,
    pub transactionReceiptUrl: Option<String>,
    pub nossoNumero: Option<String>,
    pub bankSlipUrl: Option<String>,
    pub lastInvoiceViewedDate: Option<String>,
    pub lastBankSlipViewedDate: Option<String>,
    pub discount: Discount,
    pub fine: Fine,
    pub interest: Interest,
    pub postalService: bool,
    pub escrow: Option<String>,
    pub refunds: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Discount {
    pub value: f64,
    pub limitDate: Option<String>,
    pub dueDateLimitDays: u32,
    pub r#type: DiscountType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Fine {
    pub value: f64,
    pub r#type: DiscountType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Interest {
    pub value: f64,
    pub r#type: InterestType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BillingType {
    PIX,
    BOLETO,
    CREDIT_CARD,
    UNDEFINED,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PaymentStatus {
    PENDING,
    RECEIVED,
    CONFIRMED,
    OVERDUE,
    REFUNDED,
    CANCELED,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DiscountType {
    FIXED,
    PERCENTAGE,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InterestType {
    FIXED,
    PERCENTAGE,
}