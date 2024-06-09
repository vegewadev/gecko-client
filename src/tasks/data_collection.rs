use std::error::Error;
use std::time::Duration;
use tokio::time;
use tracing::info;

use rppal::{
    gpio::{Gpio, Mode},
    hal::Delay,
};
use rppal_dht11::{Dht11, Measurement};

const GPIO_PIN: u8 = 4;

pub async fn run() -> Result<(), Box<dyn Error>> {
    info!("Starting task: data_collections");

    let pin = Gpio::new()
        .unwrap()
        .get(GPIO_PIN)
        .unwrap()
        .into_io(Mode::Output);
    let mut dht11 = Dht11::new(pin);
    let mut delay = Delay::new();

    loop {
        match dht11.perform_measurement_with_retries(&mut delay, 20) {
            Ok(Measurement {
                temperature,
                humidity,
            }) => {
                let (temperature, humidity) = (temperature as f64 / 10.0, humidity as f64 / 10.0);
                println!("Temp: {temperature:.1}C Hum: {humidity:.1}");
            }
            Err(e) => eprintln!("Failed to perform measurement: {e:?}"),
        }
        time::sleep(Duration::from_secs(10)).await;
    }
}
