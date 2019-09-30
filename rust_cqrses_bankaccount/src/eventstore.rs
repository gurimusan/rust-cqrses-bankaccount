use std::sync::Mutex;

use super::eventsourcing::{StoredEvent, EventStreamId, EventStream, EventStoreError, EventStore};
use super::aggregate::BankAccountEvent;

pub struct InmemoryBankAccountEventStore {
    events: Mutex<Vec<StoredEvent>>,
}

impl InmemoryBankAccountEventStore {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(vec![]),
        }
    }
}

impl EventStore for InmemoryBankAccountEventStore {
    type Event = BankAccountEvent;
    type EventStream = EventStream<Self::Event>;

    fn save(&self, id: EventStreamId, events: Vec<Self::Event>) -> Result<(), EventStoreError> {
        let mut guard = self.events.lock().unwrap();
        let mut i: u64 = 0;
        for event in events {
            guard.push(StoredEvent::new(
                    event.event_type().to_string(),
                    serde_json::to_string(&event).unwrap(),
                    event.occurred_at(),
                    id.stream_name().to_string(),
                    id.stream_version() + i,
                    ));
            i = i + 1;
        }
        Ok(())
    }

    fn event_stream_since(&self, id: &EventStreamId) -> Result<Self::EventStream, EventStoreError> {
        let stored_events: Vec<StoredEvent> = self.events.lock().unwrap()
            .iter()
            .filter(|event| event.stream_name() == id.stream_name() && event.stream_version() >= id.stream_version())
            .cloned()
            .collect();
        if stored_events.is_empty() {
            Err(EventStoreError::QueryError(
                    format!("There is no such event stream: {}:{}", id.stream_name(), id.stream_version())))
        } else {
            let events: Vec<BankAccountEvent> = stored_events.iter()
                .map(|event| serde_json::from_str(event.event_body()).unwrap())
                .collect();
            let version = stored_events.last().unwrap().stream_version();
            Ok(Self::EventStream::new(events, version))
        }
    }
}

#[cfg(test)]
mod tests {

    use chrono::Local;

    use super::InmemoryBankAccountEventStore;
    use super::super::eventsourcing::{StoredEvent, EventStreamId, EventStore};
    use super::super::aggregate::{BankAccountEvent, BankAccountId, BankAccountName};

    #[test]
    fn test_inmemory_store() {
        let store = InmemoryBankAccountEventStore::new();

        let bank_account_id = BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap();

        let stream_id =  EventStreamId::new(format!("bank_account:{}", bank_account_id.to_string()), 1);

        let events = vec![
            BankAccountEvent::Opened{
                bank_account_id: bank_account_id.clone(),
                name: BankAccountName::new(String::from("foo")).unwrap(),
                occurred_at: Local::now(),
            },
        ];

        let result = store.save(stream_id, events);
        assert!(result.is_ok());

        let result = store.event_stream_since(&EventStreamId::new(String::from("bank_account:67e55044-10b1-426f-9247-bb680e5fe0c8"), 1));
        assert!(result.is_ok());

        let stream = result.unwrap();
        assert_eq!(stream.events().len(), 1);
    }
}
