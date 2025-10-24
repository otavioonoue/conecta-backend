use std::error::Error;

use conecta_backend::api;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    api().await?;
    Ok(())
}
