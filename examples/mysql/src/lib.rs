#[macro_use]
extern crate diesel;

extern crate elastic;
#[macro_use]
extern crate elastic_derive;

use serde::Deserialize;

pub mod constants;
pub mod schema;
pub mod db;
pub mod eventstore;
pub mod eventpublisher;
pub mod dao;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,

    pub kafka_brokers: Vec<String>,

    pub snapshotter_kafka_consume_group: String,

    pub projector_kafka_consume_group: String,

    pub elastic_search_endpoint: String,
}
