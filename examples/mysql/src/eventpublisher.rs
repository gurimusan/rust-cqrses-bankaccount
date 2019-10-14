use std::time::Duration;
use rust_cqrses_bankaccount::eventsourcing::EventPublisher;
use rust_cqrses_bankaccount::aggregate::BankAccountEvent;

use kafka::producer::{Producer, Record, RequiredAcks};

pub struct KafkaBankAccountEventPublisher {
    hosts: Vec<String>,
    topic: String,
}

impl KafkaBankAccountEventPublisher {
    pub fn new(hosts: Vec<String>, topic: String) -> Self{
        Self {
            hosts: hosts,
            topic: topic,
        }
    }
}

impl EventPublisher for KafkaBankAccountEventPublisher {
    type Event = BankAccountEvent;

    fn publish(&self, event: Self::Event) {
        let mut producer = Producer::from_hosts(self.hosts.clone())
                 .with_ack_timeout(Duration::from_secs(1))
                 .with_required_acks(RequiredAcks::One)
                 .create()
                 .unwrap();

        producer.send(&Record {
            topic: &self.topic,
            partition: -1,
            key: (),
            value: serde_json::to_string(&event).unwrap(),
        }).unwrap();
    }
}
