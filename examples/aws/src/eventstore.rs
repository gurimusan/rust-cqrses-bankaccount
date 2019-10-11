use std::collections::HashMap;
use chrono::{Local, DateTime};
use rusoto_core::RusotoError;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, AttributeValue, PutItemInput, PutItemError, QueryInput, GetItemInput};

use rust_cqrses_bankaccount::eventsourcing::{EventStream, Snapshot, EventStoreError, EventStoreErrorKind, EventStore, EventPublisher};
use rust_cqrses_bankaccount::aggregate::{BankAccountEvent, BankAccount};

use super::eventpublisher::KafkaBankAccountEventPublisher;

pub struct DynamoDbBankAccountEventStore {
    client: DynamoDbClient,
    publisher: KafkaBankAccountEventPublisher,
}

impl DynamoDbBankAccountEventStore {
    pub fn new(client: DynamoDbClient, publisher: KafkaBankAccountEventPublisher) -> Self {
        Self {
            client: client,
            publisher: publisher,
        }
    }
}

impl EventStore for DynamoDbBankAccountEventStore {
    type Event = BankAccountEvent;
    type EventStream = EventStream<Self::Event>;
    type SnapshotData = BankAccount;

    fn append_event_stream(&self, stream_id: String, stream_version: u64, events: Vec<Self::Event>) -> Result<(), EventStoreError> {
        let mut i: u64 = 0;
        let mut stream_version = stream_version;
        for event in events {
            stream_version = stream_version + i;
            let mut item: HashMap<String, AttributeValue> = HashMap::new();
            item.insert(String::from("stream_id"), AttributeValue {
                s: Some(stream_id.clone()),
                ..Default::default()
            });
            item.insert(String::from("stream_version"), AttributeValue {
                n: Some(stream_version.to_string()),
                ..Default::default()
            });
            item.insert(String::from("event_type"), AttributeValue {
                s: Some(event.event_type().to_string()),
                ..Default::default()
            });
            item.insert(String::from("event_body"), AttributeValue {
                s: Some(serde_json::to_string(&event).unwrap()),
                ..Default::default()
            });
            item.insert(String::from("event_occurred_at"), AttributeValue {
                s: Some(event.occurred_at().to_rfc3339()),
                ..Default::default()
            });

            let input = PutItemInput {
                item: item,
                table_name: String::from("event_store"),
                condition_expression: Some(format!(
                    "{} OR {} AND {}",
                    "attribute_not_exists(stream_id)",
                    "attribute_not_exists(stream_id)",
                    "attribute_not_exists(stream_version)",
                    )),
                ..Default::default()
            };

            match self.client.put_item(input).sync() {
                Ok(_) => {
                    self.publisher.publish(event.clone());
                },
                Err(err) => {
                    return match err {
                        RusotoError::Service(PutItemError::ConditionalCheckFailed(_)) =>  {
                            Err(EventStoreErrorKind::DuplicateEntryError(err.to_string()))?
                        },
                        _ => Err(EventStoreErrorKind::AppendEventStreamError(err.to_string()))?,
                    };
                }
            };

            i = i + 1;
        }

        Ok(())
    }

    fn event_stream_since(&self, stream_id: String, stream_version: u64) -> Result<Self::EventStream, EventStoreError> {
        let key_conditions = vec![
            String::from("stream_id = :stream_id"),
            String::from("stream_version >= :stream_version"),
        ];

        let mut values: HashMap<String, AttributeValue> = HashMap::new();
        values.insert(String::from(":stream_id"), AttributeValue {
            s: Some(stream_id.clone()),
            ..Default::default()
        });
        values.insert(String::from(":stream_version"), AttributeValue {
            n: Some(stream_version.to_string()),
            ..Default::default()
        });

        let input = QueryInput {
            table_name: String::from("event_store"),
            key_condition_expression: Some(key_conditions.join(" AND ")),
            expression_attribute_values: Some(values),
            ..Default::default()
        };

        match self.client.query(input).sync() {
            Ok(output) => {
                if output.count.unwrap() == 0 {
                    return Err(EventStoreErrorKind::NoEventStreamError(stream_id.clone(), stream_version))?;
                }
                let events = output.items.as_ref().unwrap()
                    .iter()
                    .map(|attributes| {
                        serde_json::from_str(&attributes.get("event_body").unwrap().s.as_ref().unwrap()).unwrap()
                    })
                    .collect();
                let version = output.items.as_ref().unwrap()
                    .last().unwrap()
                    .get("stream_version").unwrap().n.as_ref().unwrap().clone()
                    .parse().unwrap();
                Ok(EventStream::new(events, version))
            },
            Err(err) => Err(EventStoreErrorKind::QueryError(err.to_string()))?,
        }
    }

    fn record_snapshot(&self, snapshot: Snapshot<Self::SnapshotData>) -> Result<(), EventStoreError> {
        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        item.insert(String::from("stream_id"), AttributeValue {
            s: Some(snapshot.stream_id().to_string()),
            ..Default::default()
        });
        item.insert(String::from("stream_version"), AttributeValue {
            n: Some(snapshot.stream_version().to_string()),
            ..Default::default()
        });
        item.insert(String::from("snapshot"), AttributeValue {
            s: Some(serde_json::to_string(snapshot.snapshot()).unwrap()),
            ..Default::default()
        });
        item.insert(String::from("created_at"), AttributeValue {
            s: Some(snapshot.created_at().to_rfc3339()),
            ..Default::default()
        });

        let input = PutItemInput {
            item: item,
            table_name: String::from("snapshot"),
            ..Default::default()
        };

        match self.client.put_item(input).sync() {
            Ok(_) => Ok(()),
            Err(err) => Err(EventStoreErrorKind::AppendEventStreamError(err.to_string()))?,
        }
    }

    fn read_snapshot(&self, stream_id: String) -> Result<Option<Snapshot<Self::SnapshotData>>, EventStoreError> {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();

        key.insert(String::from("stream_id"), AttributeValue {
            s: Some(stream_id.clone()),
            ..Default::default()
        });

        let input = GetItemInput {
            table_name: String::from("snapshot"),
            key: key,
            ..Default::default()
        };

        match self.client.get_item(input).sync() {
            Ok(output) => {
                match output.item {
                    Some(attributes) => {
                        let stream_id = attributes.get("stream_id").unwrap()
                            .s.as_ref().unwrap()
                            .clone();
                        let stream_version = attributes.get("stream_version").unwrap()
                            .n.as_ref().unwrap().clone()
                            .parse().unwrap();
                        let snapshot = serde_json::from_str(
                            &attributes.get("snapshot").unwrap().s.as_ref().unwrap()).unwrap();
                        let created_at = DateTime::parse_from_rfc3339(
                            &attributes.get("created_at").unwrap().s.as_ref().unwrap()
                            ).unwrap()
                            .with_timezone(&Local);
                        Ok(Some(Snapshot::new(
                            stream_id,
                            stream_version,
                            snapshot,
                            created_at,
                            )))
                    },
                    None => Ok(None),
                }
            },
            Err(err) => Err(EventStoreErrorKind::QueryError(err.to_string()))?,
        }
    }
}
