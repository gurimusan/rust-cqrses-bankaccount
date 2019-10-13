#[macro_use]
extern crate diesel;

use serde::Deserialize;

pub mod constants;
pub mod schema;
pub mod db;
pub mod eventstore;
pub mod eventpublisher;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,

    pub kafka_brokers: Vec<String>,

    pub kafka_consume_group: String,
}
