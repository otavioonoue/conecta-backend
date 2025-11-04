use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AddServiceDto {
    pub service_id: String
}