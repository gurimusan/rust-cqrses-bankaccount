use log::{info, error};
use failure;
use structopt::StructOpt;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

use rust_cqrses_bankaccount::snapshotter::BankAccountAggregateSnapshotter;
use rust_cqrses_bankaccount::aggregate::BankAccountEvent;

use rust_cqrses_bankaccount_aws_example::constants;
use rust_cqrses_bankaccount_aws_example::eventstore::DynamoDbBankAccountEventStore;
use rust_cqrses_bankaccount_aws_example::eventpublisher::KafkaBankAccountEventPublisher;

fn main() {
    env_logger::init();

    let config = Config::from_args();

    consume_messages(config);
}

#[derive(StructOpt, Debug)]
#[structopt(name = "snapshot_runner")]
struct Config {
    #[structopt(long)]
    dynamodb_endpoint: String,

    #[structopt(long)]
    dynamodb_region: String,

    #[structopt(long, required = true)]
    kafka_brokers: Vec<String>,

    #[structopt(long)]
    kafka_consume_group: String,

    #[structopt(long)]
    dryrun: bool,
}

fn consume_messages(config: Config) {
    let region = Region::Custom {
        name: config.dynamodb_region.clone(),
        endpoint: config.dynamodb_endpoint.clone()
    };

    let client = DynamoDbClient::new(region);

    let eventpublisher = KafkaBankAccountEventPublisher::new(config.kafka_brokers.clone());

    let eventstore = Box::new(DynamoDbBankAccountEventStore::new(client, eventpublisher));

    let snapshotter = BankAccountAggregateSnapshotter::new(eventstore);

    let mut con = {
        let cb = Consumer::from_hosts(config.kafka_brokers.clone())
                .with_group(config.kafka_consume_group.clone())
                .with_topic(String::from(constants::TOPIC))
                .with_fallback_offset(FetchOffset::Earliest)
                .with_offset_storage(GroupOffsetStorage::Kafka);
        cb.create().unwrap()
    };

    loop {
        let mss = match con.poll() {
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
                info!("{}:{}@{}: {:?}", ms.topic(), ms.partition(), m.offset, &event);
                match event {
                    BankAccountEvent::Opened{ bank_account_id, name: _, occurred_at: _ } => {
                        snapshotter.take_snapshot(bank_account_id.clone());
                    },
                    BankAccountEvent::Updated{ bank_account_id, name: _, occurred_at: _ } => {
                        snapshotter.take_snapshot(bank_account_id.clone());
                    },
                    BankAccountEvent::Deposited{ bank_account_id, deposit: _, occurred_at: _ } => {
                        snapshotter.take_snapshot(bank_account_id.clone());
                    },
                    BankAccountEvent::Withdrawn{ bank_account_id, withdraw: _, occurred_at: _ } => {
                        snapshotter.take_snapshot(bank_account_id.clone());
                    },
                    BankAccountEvent::Closed{ bank_account_id, occurred_at: _ } => {
                        snapshotter.take_snapshot(bank_account_id.clone());
                    },
                };
            }
            if error_occrred {
                break;
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed().unwrap();
    }
}
