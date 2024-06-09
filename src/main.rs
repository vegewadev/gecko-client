mod database;

use database::database::Database;
use dotenv::dotenv;
use mongodb::error::Result;
use tokio;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber: FmtSubscriber = FmtSubscriber::builder().finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    dotenv().ok();

    let connection_string = std::env::var("CONNECTION_STRING")
        .expect("CONNECTION_STRING must be set as an environment variable.");

    match Database::connect(connection_string).await {
        Ok(db) => {
            if let Some(_client) = db.client {
                info!("[INFO] Database connected successfully");
            } else {
                error!("[ERROR] Client is None after successful connection");
            }
        }
        Err(e) => {
            error!("[ERROR] Failed to connect to the database: {:?}", e);
            return Err(e);
        }
    }

    Ok(())
}
