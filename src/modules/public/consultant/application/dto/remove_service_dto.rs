use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RemoveServiceDto {
    pub service_id: String
}