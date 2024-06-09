use std::error::Error;
use tokio::time;
use tracing::info;

use dht11_gpio::{DHT11Controller, Sensor};

pub async fn run() -> Result<(), Box<dyn Error>> {
    info!("Starting task: data_collections");

    const DHT11_PIN: u8 = 4;

    let mut sensor = DHT11Controller::new(DHT11_PIN).unwrap();

    let result = sensor.read_sensor_data();
    match result {
        Ok(data) => {
            println!("temperature: {} °C", data.temperature);
            println!("humidity: {} %", data.humidity);
            Ok(())
        }
        Err(err) => {
            println!("error: {}", err);
            Err(Box::new(err))
        }
    }

    // loop {
    //     time::sleep(Duration::from_secs(10)).await;
    // }
}
