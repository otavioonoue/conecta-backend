use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureException {
    #[error("Database error: {0}")]
    InternalDatabaseError(String)
}