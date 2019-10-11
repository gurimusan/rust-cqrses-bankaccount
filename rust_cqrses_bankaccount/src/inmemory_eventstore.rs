use std::sync::Mutex;
use std::collections::HashMap;
use chrono::{DateTime, Local};

use super::eventsourcing::{EventStream, Snapshot, EventStoreError, EventStoreErrorKind, EventStore};
use super::aggregate::{BankAccountEvent, BankAccount};

#[derive(Debug, Clone)]
pub struct StoredEvent {
    event_type: String,
    event_body: String,
    event_occurred_at: DateTime<Local>,
    stream_id: String,
    stream_version: u64,
}

impl StoredEvent {
    pub fn new(
        event_type: String,
        event_body: String,
        event_occurred_at: DateTime<Local>,
        stream_id: String,
        stream_version: u64,
        ) -> Self {
        Self {
            event_type,
            event_body,
            event_occurred_at,
            stream_id,
            stream_version,
        }
    }

    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    pub fn event_body(&self) -> &str {
        &self.event_body
    }

    pub fn event_occurred_at(&self) -> &DateTime<Local> {
        &self.event_occurred_at
    }

    pub fn stream_id(&self) -> &str {
        &self.stream_id
    }

    pub fn stream_version(&self) -> u64 {
        self.stream_version
    }
}

pub struct InmemoryBankAccountEventStore {
    events: Mutex<Vec<StoredEvent>>,
    snapshots: Mutex<HashMap<String, Snapshot<BankAccount>>>,
}

impl InmemoryBankAccountEventStore {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(vec![]),
            snapshots: Mutex::new(HashMap::new()),
        }
    }
}

impl EventStore for InmemoryBankAccountEventStore {
    type Event = BankAccountEvent;
    type EventStream = EventStream<Self::Event>;
    type SnapshotData = BankAccount;

    fn append_event_stream(&self, stream_id: String, stream_version: u64, events: Vec<Self::Event>)
        -> Result<(), EventStoreError> {
        let mut guard = self.events.lock().unwrap();
        let mut i: u64 = 0;
        let mut stream_version = stream_version;
        for event in events {
            stream_version = stream_version + i;
            guard.push(StoredEvent::new(
                    event.event_type().to_string(),
                    serde_json::to_string(&event).unwrap(),
                    event.occurred_at(),
                    stream_id.clone(),
                    stream_version,
                    ));
            i = i + 1;
        }
        Ok(())
    }

    fn event_stream_since(&self, stream_id: String, stream_version: u64)
        -> Result<Self::EventStream, EventStoreError> {
        let stored_events: Vec<StoredEvent> = self.events.lock().unwrap()
            .iter()
            .filter(|event| event.stream_id() == &stream_id && event.stream_version() >= stream_version)
            .cloned()
            .collect();
        if stored_events.is_empty() {
            Err(EventStoreErrorKind::QueryError(
                    format!("There is no such event stream: {}:{}", &stream_id, stream_version)))?
        } else {
            let events: Vec<BankAccountEvent> = stored_events.iter()
                .map(|event| serde_json::from_str(event.event_body()).unwrap())
                .collect();
            let version = stored_events.last().unwrap().stream_version();
            Ok(Self::EventStream::new(events, version))
        }
    }

    fn record_snapshot(&self, snapshot: Snapshot<Self::SnapshotData>) -> Result<(), EventStoreError> {
        let mut guard = self.snapshots.lock().unwrap();
        guard.insert(snapshot.stream_id().to_string(), snapshot);
        Ok(())
    }

    fn read_snapshot(&self, stream_id: String) -> Result<Option<Snapshot<Self::SnapshotData>>, EventStoreError> {
        let guard = self.snapshots.lock().unwrap();
        match guard.get(&stream_id) {
            Some(snapshot) => Ok(Some(snapshot.clone())),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {

    use chrono::Local;

    use super::InmemoryBankAccountEventStore;
    use super::super::eventsourcing::EventStore;
    use super::super::aggregate::{BankAccountEvent, BankAccountId, BankAccountName};

    #[test]
    fn test_inmemory_store() {
        let store = InmemoryBankAccountEventStore::new();

        let bank_account_id = BankAccountId::new(String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")).unwrap();

        let stream_id = format!("bank_account:{}", bank_account_id.to_string());

        let events = vec![
            BankAccountEvent::Opened{
                bank_account_id: bank_account_id.clone(),
                name: BankAccountName::new(String::from("foo")).unwrap(),
                occurred_at: Local::now(),
            },
        ];

        let result = store.append_event_stream(stream_id, 1, events);
        assert!(result.is_ok());

        let result = store.event_stream_since(String::from("bank_account:67e55044-10b1-426f-9247-bb680e5fe0c8"), 1);
        assert!(result.is_ok());

        let stream = result.unwrap();
        assert_eq!(stream.events().len(), 1);
    }
}
