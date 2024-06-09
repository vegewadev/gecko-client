use mongodb::{error::Result, options::ClientOptions, Client};
use tracing::{error, info};

pub struct Database {
    pub connection_string: String,
    pub client: Option<Client>,
}

impl Database {
    pub async fn connect(connection_string: String) -> Result<Self> {
        let client_options: ClientOptions = match ClientOptions::parse(&connection_string).await {
            Ok(options) => options,
            Err(e) => {
                error!("Failed to parse client options: {:?}", e);
                return Err(e.into());
            }
        };

        let client = match Client::with_options(client_options) {
            Ok(client) => {
                info!("Successfully created client with options");
                client
            }
            Err(e) => {
                error!("Failed to create client with options: {:?}", e);
                return Err(e.into());
            }
        };

        // Test database connection
        match client.list_database_names(None, None).await {
            Ok(_) => {
                info!("Database connection test succeeded");
                Ok(Database {
                    connection_string,
                    client: Some(client),
                })
            }
            Err(e) => {
                error!(
                    "Database connection test failed. Failed to list database names: {:?}",
                    e
                );
                Err(e.into())
            }
        }
    }
}
