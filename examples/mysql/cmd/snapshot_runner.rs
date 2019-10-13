use log::{info, error};
use structopt::StructOpt;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

use rust_cqrses_bankaccount::snapshotter::BankAccountAggregateSnapshotter;
use rust_cqrses_bankaccount::aggregate::BankAccountEvent;

use rust_cqrses_bankaccount_mysql_example::Config;
use rust_cqrses_bankaccount_mysql_example::constants;
use rust_cqrses_bankaccount_mysql_example::db;
use rust_cqrses_bankaccount_mysql_example::eventstore::MysqlBankAccountEventStore;
use rust_cqrses_bankaccount_mysql_example::eventpublisher::KafkaBankAccountEventPublisher;

fn main() {
    dotenv::dotenv().ok();

    env_logger::init();

    let config = envy::from_env::<Config>().unwrap();

    let args = Args::from_args();

    consume_messages(args, config);
}

#[derive(StructOpt, Debug)]
#[structopt(name = "snapshot_runner")]
struct Args {
    #[structopt(long)]
    dryrun: bool,
}

fn consume_messages(args: Args, config: Config) {
    let pool = db::init_database_pool(&config.database_url);

    let eventpublisher = KafkaBankAccountEventPublisher::new(config.kafka_brokers.clone());

    let eventstore = Box::new(MysqlBankAccountEventStore::new(pool, eventpublisher));

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
                if !args.dryrun {
                    let result = match &event {
                        BankAccountEvent::Opened{ bank_account_id, name: _, occurred_at: _ } => {
                            snapshotter.take_snapshot(bank_account_id.clone())
                        },
                        BankAccountEvent::Updated{ bank_account_id, name: _, occurred_at: _ } => {
                            snapshotter.take_snapshot(bank_account_id.clone())
                        },
                        BankAccountEvent::Deposited{ bank_account_id, deposit: _, occurred_at: _ } => {
                            snapshotter.take_snapshot(bank_account_id.clone())
                        },
                        BankAccountEvent::Withdrawn{ bank_account_id, withdraw: _, occurred_at: _ } => {
                            snapshotter.take_snapshot(bank_account_id.clone())
                        },
                        BankAccountEvent::Closed{ bank_account_id, occurred_at: _ } => {
                            snapshotter.take_snapshot(bank_account_id.clone())
                        },
                    };
                    match result {
                        Ok(_) => {},
                        Err(err) => println!("Snapshot error: {:?}", err.to_string()),
                    }
                }
                info!("{}:{}@{}: {:?}", ms.topic(), ms.partition(), m.offset, &event);
            }
            if error_occrred {
                break;
            }
            let _ = con.consume_messageset(ms);
        }
        if !args.dryrun {
            con.commit_consumed().unwrap();
        }
    }
}
