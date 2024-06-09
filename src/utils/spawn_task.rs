use crate::tasks::data_collection;

use tokio;

pub async fn spawn_task_data_collection() {
    tokio::spawn(async {
        data_collection::run().await;
    });
}
