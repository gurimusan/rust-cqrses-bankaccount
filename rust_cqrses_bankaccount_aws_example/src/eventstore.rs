use std::collections::HashMap;
use rusoto_core::RusotoError;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, AttributeValue, PutItemInput, PutItemError, QueryInput};

use rust_cqrses_bankaccount::eventsourcing::{EventStreamId, EventStream, EventStoreError, EventStore};
use rust_cqrses_bankaccount::aggregate::BankAccountEvent;

pub struct DynamoDbBankAccountEventStore {
    client: DynamoDbClient,
}

impl DynamoDbBankAccountEventStore {
    pub fn new(client: DynamoDbClient) -> Self {
        Self {
            client: client,
        }
    }
}

impl EventStore for DynamoDbBankAccountEventStore {
    type Event = BankAccountEvent;
    type EventStream = EventStream<Self::Event>;

    fn save(&self, id: EventStreamId, events: Vec<Self::Event>) -> Result<(), EventStoreError> {
        let mut i: u64 = 0;
        for event in events {
            let mut item: HashMap<String, AttributeValue> = HashMap::new();
            item.insert(String::from("stream_id"), AttributeValue {
                s: Some(id.stream_name().to_string()),
                ..Default::default()
            });
            item.insert(String::from("stream_version"), AttributeValue {
                n: Some((id.stream_version() + i).to_string()),
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
                Ok(_) => {},
                Err(err) => {
                    return match err {
                        RusotoError::Service(PutItemError::ConditionalCheckFailed(_)) =>  {
                            Err(EventStoreError::DuplicateEntryError(err.to_string()))
                        },
                        _ => Err(EventStoreError::SaveError(err.to_string())),
                    };
                }
            };

            i = i + 1;
        }

        Ok(())
    }

    fn event_stream_since(&self, id: &EventStreamId) -> Result<Self::EventStream, EventStoreError> {
        let key_conditions = vec![
            String::from("stream_id = :stream_id"),
            String::from("stream_version >= :stream_version"),
        ];

        let mut values: HashMap<String, AttributeValue> = HashMap::new();
        values.insert(String::from(":stream_id"), AttributeValue {
            s: Some(id.stream_name().to_string()),
            ..Default::default()
        });
        values.insert(String::from(":stream_version"), AttributeValue {
            n: Some((id.stream_version()).to_string()),
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
                    return Err(EventStoreError::NoEventStreamError(id.stream_name().to_string(), id.stream_version()));
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
            Err(err) => Err(EventStoreError::QueryError(err.to_string())),
        }
    }
}
