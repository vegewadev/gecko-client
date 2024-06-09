use crate::tasks::data_collection;

use mongodb::Client;
use tokio;

pub async fn spawn_task_data_collection(client: Client) {
    tokio::spawn(async {
        data_collection::run(client).await;
    });
}
