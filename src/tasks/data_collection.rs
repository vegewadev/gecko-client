use chrono::prelude::*;
use mongodb::{bson::DateTime as BsonDateTime, Client, Collection};
use serde::Serialize;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

use dht11_gpio::{DHT11Controller, Sensor};

#[derive(Serialize)]
struct ClimateMonitoring {
    device_id: String,
    interval_start: BsonDateTime,
    data: Vec<ClimateData>,
    units: Units,
    metadata: Metadata,
}

#[derive(Serialize)]
struct ClimateData {
    timestamp: BsonDateTime,
    temperature: f64,
    humidity: f64,
}

#[derive(Serialize)]
struct Units {
    temperature: String,
    humidity: String,
}

#[derive(Serialize)]
struct Metadata {
    sensor_type: String,
    installation_date: BsonDateTime,
}

pub async fn run(client: Client) {
    info!("Starting task: data_collections");

    const DHT11_PIN: u8 = 4;
    let collection: Collection<ClimateMonitoring> = client
        .database("gecko-client")
        .collection("environmental_information");

    loop {
        let mut sensor: DHT11Controller = DHT11Controller::new(DHT11_PIN).unwrap();

        let result: Result<dht11_gpio::DHT11Result, dht11_gpio::DHT11Error> =
            sensor.read_sensor_data();
        match result {
            Ok(data) => {
                info!("captured temperature: {} °C", data.temperature);
                info!("captured humidity: {} %", data.humidity);

                let now: DateTime<Utc> = Utc::now();
                let now_with_offset: DateTime<Utc> = now.with_timezone(&Utc);
                let millis_since_epoch: i64 = now_with_offset.timestamp_millis();

                let monitoring_data: ClimateMonitoring = ClimateMonitoring {
                    device_id: String::from("sensor-001"),
                    interval_start: BsonDateTime::from_millis(millis_since_epoch),
                    data: vec![ClimateData {
                        timestamp: BsonDateTime::from_millis(millis_since_epoch),
                        temperature: data.temperature,
                        humidity: data.humidity,
                    }],
                    units: Units {
                        temperature: String::from("Celsius"),
                        humidity: String::from("Percentage"),
                    },
                    metadata: Metadata {
                        sensor_type: String::from("DHT22"),
                        installation_date: BsonDateTime::from_millis(
                            Utc.ymd(2023, 1, 1).and_hms(0, 0, 0).timestamp_millis(),
                        ),
                    },
                };

                let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
                    collection.insert_one(monitoring_data, None).await;
                match result {
                    Ok(_) => info!("Data inserted successfully."),
                    Err(e) => error!("Error inserting data: {}", e),
                }
            }
            Err(err) => {
                error!("error capturing temperature and humidity: {}", err);
            }
        }
        time::sleep(Duration::from_secs(10)).await;
    }
}
