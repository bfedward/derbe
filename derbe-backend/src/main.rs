use dotenv::dotenv;

use derbe_backend::{configuration::get_configuration, http};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let configuration = get_configuration().expect("Failed to read configuration.");

    http::serve(configuration).await
}
