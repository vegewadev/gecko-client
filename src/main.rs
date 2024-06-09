mod database;
mod tasks;
mod utils;

use std::time::Duration;

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
            if let Some(client) = db.client {
                info!("Database connected successfully");

                info!("Spawning task: data_collection");
                utils::spawn_task::spawn_task_data_collection(client).await;
            } else {
                error!("Client is None after successful connection");
            }
        }
        Err(e) => {
            error!("Failed to connect to the database: {:?}", e);
            return Err(e);
        }
    }

    // Program loop so it doesn't exit
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
