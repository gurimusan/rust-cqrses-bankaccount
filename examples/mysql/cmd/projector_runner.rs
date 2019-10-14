use log::{info, error};

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use elastic::prelude::*;

use rust_cqrses_bankaccount::aggregate::BankAccountEvent;
use rust_cqrses_bankaccount::projector::BankAccountProjector;
use rust_cqrses_bankaccount_mysql_example::Config;
use rust_cqrses_bankaccount_mysql_example::constants;
use rust_cqrses_bankaccount_mysql_example::dao::ElasticBankAccountDao;

fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    let config = envy::from_env::<Config>().unwrap();

    project(config);
}

fn project(config: Config) {
    let client = SyncClient::builder()
        .static_node(config.elastic_search_endpoint)
        .build()
        .unwrap();

    let dao = Box::new(ElasticBankAccountDao::new(client));

    let projector = BankAccountProjector::new(dao);

    let mut consumer = {
        let cb = Consumer::from_hosts(config.kafka_brokers.clone())
                .with_group(config.projector_kafka_consume_group.clone())
                .with_topic(String::from(constants::TOPIC))
                .with_fallback_offset(FetchOffset::Earliest)
                .with_offset_storage(GroupOffsetStorage::Kafka);
        cb.create().unwrap()
    };

    loop {
        let mss = match consumer.poll() {
            Ok(mss) => mss,
            Err(err) => {
                error!("Error occrred: {:?}", err);
                continue;
            }
        };

        if mss.is_empty() {
            continue;
        }

        let mut error_occrred = false;

        for ms in mss.iter() {
            for m in ms.messages() {
                let event: BankAccountEvent = match serde_json::from_slice(m.value) {
                    Ok(event) => event,
                    Err(err) => {
                        error!("Serialize event error: {:?}", err);
                        error_occrred = true;
                        break;
                    }
                };

                projector.project(event.clone());

                info!("{}:{}@{}: {:?}", ms.topic(), ms.partition(), m.offset, &event);
            }
            if error_occrred {
                break;
            }
            let _ = consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }

}
