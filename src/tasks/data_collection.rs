use std::time::Duration;
use tokio::time;
use tracing::{error, info};

use dht11_gpio::{DHT11Controller, Sensor};

pub async fn run() {
    info!("Starting task: data_collections");

    const DHT11_PIN: u8 = 4;

    loop {
        let mut sensor: DHT11Controller = DHT11Controller::new(DHT11_PIN).unwrap();

        let result: Result<dht11_gpio::DHT11Result, dht11_gpio::DHT11Error> =
            sensor.read_sensor_data();
        match result {
            Ok(data) => {
                info!("captured temperature: {} °C", data.temperature);
                info!("captured humidity: {} %", data.humidity);
            }
            Err(err) => {
                error!("error capturing temperature and humidity: {}", err);
            }
        }
        time::sleep(Duration::from_secs(10)).await;
    }
}
