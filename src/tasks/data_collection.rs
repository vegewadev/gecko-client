use chrono::prelude::*;
use mongodb::{
    bson::{self, doc, DateTime as BsonDateTime},
    options::{FindOneOptions, UpdateModifications, UpdateOptions},
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

use dht11_gpio::{DHT11Controller, Sensor};

#[derive(Serialize, Deserialize, Debug)]
struct ClimateMonitoring {
    device_id: String,
    interval_start: BsonDateTime,
    data: Vec<ClimateData>,
    units: Units,
    metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClimateData {
    timestamp: BsonDateTime,
    temperature: f64,
    humidity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Units {
    temperature: String,
    humidity: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    sensor_type: String,
    installation_date: BsonDateTime,
}

async fn check_interval_start_in_last_120_minutes(
    collection: Collection<ClimateMonitoring>,
) -> mongodb::error::Result<(bool, Option<ClimateMonitoring>)> {
    let current_time: DateTime<Utc> = Utc::now();
    let time_120_minutes_ago: DateTime<Utc> = current_time - chrono::Duration::minutes(120);

    let current_time_millis: i64 = current_time.timestamp_millis();
    let time_120_minutes_ago_millis: i64 = time_120_minutes_ago.timestamp_millis();

    let filter: mongodb::bson::Document = doc! {
        "interval_start": {
            "$gte": BsonDateTime::from_millis(time_120_minutes_ago_millis),
            "$lte": BsonDateTime::from_millis(current_time_millis),
        }
    };

    let find_options = FindOneOptions::builder()
        .sort(doc! { "interval_start": -1 })
        .build();

    let result: Option<ClimateMonitoring> = collection.find_one(filter, find_options).await?;

    let is_within_last_120_minutes = result.is_some();

    Ok((is_within_last_120_minutes, result))
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

                match check_interval_start_in_last_120_minutes(collection.clone()).await {
                    Ok((is_within_last_120_minutes, result)) => {
                        info!("Is within last 120 minutes: {}", is_within_last_120_minutes);

                        let now: DateTime<Utc> = Utc::now();
                        let now_with_offset: DateTime<Utc> = now.with_timezone(&Utc);
                        let millis_since_epoch: i64 = now_with_offset.timestamp_millis();

                        if let Some(doc) = result {
                            info!("Document found within last 120 minutes, updating");

                            let modification: UpdateModifications = UpdateModifications::Document(
                                doc! {
                                    "$push": {
                                        "data": {
                                            "timestamp": BsonDateTime::from_millis(millis_since_epoch),
                                            "temperature": data.temperature,
                                            "humidity": data.humidity,
                                        }
                                    }
                                },
                            );

                            let update_options: UpdateOptions = UpdateOptions::builder().build();

                            let result: Result<
                                mongodb::results::UpdateResult,
                                mongodb::error::Error,
                            > = collection
                                .update_one(
                                    doc! { "interval_start": doc.interval_start },
                                    modification,
                                    update_options,
                                )
                                .await;

                            match result {
                                Ok(update_result) => {
                                    if update_result.matched_count == 1 {
                                        info!("Document updated successfully");
                                    } else {
                                        info!("No document matched the filter");
                                    }
                                }
                                Err(err) => {
                                    error!("Error updating document: {}", err);
                                }
                            }
                        } else {
                            info!("No document found within last 120 minutes, creating a new one");

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
                                    sensor_type: String::from("DHT11"),
                                    installation_date: BsonDateTime::from_millis(
                                        Utc.ymd(2024, 6, 9).and_hms(0, 0, 0).timestamp_millis(),
                                    ),
                                },
                            };

                            let result: Result<
                                mongodb::results::InsertOneResult,
                                mongodb::error::Error,
                            > = collection.insert_one(monitoring_data, None).await;
                            match result {
                                Ok(_) => info!("Data inserted successfully."),
                                Err(e) => error!("Error inserting data: {}", e),
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
            }
            Err(err) => {
                error!("error capturing temperature and humidity: {}", err);
            }
        }
        time::sleep(Duration::from_secs(10)).await;
    }
}
