use std::error::Error;

use derbe_backend::{configuration::get_configuration, http};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Loads `.env` too, so RUST_LOG from it is set before the subscriber reads it.
    let configuration = get_configuration()?;
    tracing_subscriber::fmt::init();

    http::serve(configuration).await?;
    Ok(())
}
