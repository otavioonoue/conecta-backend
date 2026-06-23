use serde::Deserialize;

#[derive(Deserialize)]
pub struct FindPaymentDto {
    pub kind: KindPaymentDto
}

#[derive(Deserialize)]
pub enum KindPaymentDto {
    SCHEDULED,
    BUDGET
}

impl ToString for KindPaymentDto {
    fn to_string(&self) -> String {
        match self {
            KindPaymentDto::SCHEDULED => String::from("SCHEDULED"),
            KindPaymentDto::BUDGET => String::from("BUDGET"),
        }
    }
}