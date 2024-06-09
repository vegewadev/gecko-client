use std::time::Duration;
use tracing::info;

use tokio::time;

pub async fn run() {
    loop {
        info!("Saving...");
        time::sleep(Duration::from_secs(10)).await;
    }
}
