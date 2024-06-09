use dht_sensor::Sensor;
use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use tracing::info;

const GPIO_PIN: u8 = 4;

pub async fn run() -> Result<(), Box<dyn Error>> {
    info!("Starting task: data_collections");

    let gpio = Gpio::new()?;
    let mut information = gpio.get(GPIO_PIN)?.into_output();

    let sensor = Sensor::new(GPIO_PIN)?;

    loop {
        match sensor.read() {
            Ok(data) => {
                info!(
                    "Temperature: {:.1}°C, Humidity: {:.1}%",
                    data.temperature, data.humidity
                );
            }
            Err(e) => {
                info!("Error reading from sensor: {}", e);
            }
        }

        information.toggle();

        time::sleep(Duration::from_secs(10)).await;
    }
}
