use dotenv::dotenv;
use std::error::Error;

use derbe_backend::{configuration::get_configuration, http};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let configuration = get_configuration()?;

    http::serve(configuration).await?;
    Ok(())
}
