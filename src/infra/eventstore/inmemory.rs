use std::sync::Mutex;
use chrono::{DateTime, Local};

use super::super::super::eventstore::{Event, EventStreamId, EventStream, EventStoreError, EventStore};

#[derive(Debug, Clone)]
struct StoredEvent {
    stream_name: String,
    stream_version: u64,
    event_type: String,
    event_body: String,
    event_occurred_at: DateTime<Local>,
}

impl StoredEvent {
    pub fn new(
        stream_name: String,
        stream_version: u64,
        event_type: String,
        event_body: String,
        event_occurred_at: DateTime<Local>) -> Self {
        Self {
            stream_name: stream_name,
            stream_version: stream_version,
            event_type: event_type,
            event_body: event_body,
            event_occurred_at: event_occurred_at,
        }
    }

    pub fn stream_name(&self) -> &str {
        &self.stream_name
    }

    pub fn stream_version(&self) -> u64 {
        self.stream_version
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
}

pub struct InmemoryEventStore {
    events: Mutex<Vec<StoredEvent>>,
}

impl InmemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(vec![]),
        }
    }
}

impl EventStore for InmemoryEventStore {
    fn save(&self, id: EventStreamId, events: Vec<Event>) -> Result<(), EventStoreError> {
        let mut guard = self.events.lock().unwrap();
        let mut i: u64 = 0;
        for event in events {
            guard.push(StoredEvent::new(
                    id.stream_name().to_string(),
                    id.stream_version() + i,
                    event.event_type().to_string(),
                    event.event_body().to_string(),
                    event.event_occurred_at().clone(),
                    ));
            i = i + 1;
        }
        Ok(())
    }

    fn event_stream_since(&self, id: &EventStreamId) -> Result<EventStream, EventStoreError> {
        let stored_events: Vec<StoredEvent> = self.events.lock().unwrap()
            .iter()
            .filter(|event| event.stream_name() == id.stream_name() && event.stream_version() >= id.stream_version())
            .cloned()
            .collect();
        if stored_events.is_empty() {
            Err(EventStoreError::QueryError(
                    format!("There is no such event stream: {}:{}", id.stream_name(), id.stream_version())))
        } else {
            let events: Vec<Event> = stored_events.iter()
                .map(|event| Event::new(
                        event.event_type().to_string(),
                        event.event_body().to_string(),
                        event.event_occurred_at().clone(),
                        ))
                .collect();
            let version = stored_events.last().unwrap().stream_version();
            Ok(EventStream::new(events, version))
        }
    }
}

#[cfg(test)]
mod tests {

    use chrono::Local;

    use super::InmemoryEventStore;
    use super::super::super::super::eventstore::{Event, EventStreamId, EventStore};

    #[test]
    fn test_inmemory_store() {
        let store = InmemoryEventStore::new();

        let stream_id =  EventStreamId::new(String::from("foo:1"), 1);

        let events = vec![
            Event::new(
                String::from("FooCreated"),
                serde_json::json!({
                    "name": "foo",
                }).to_string(),
                Local::now(),
            ),
            Event::new(
                String::from("FooUpdated"),
                serde_json::json!({
                    "name": "foo updated",
                }).to_string(),
                Local::now(),
            ),
            Event::new(
                String::from("FooUpdated"),
                serde_json::json!({
                    "name": "foo updated 2",
                }).to_string(),
                Local::now(),
            ),
        ];

        let result = store.save(stream_id, events);
        assert!(result.is_ok());

        let result = store.event_stream_since(&EventStreamId::new(String::from("foo:1"), 1));
        assert!(result.is_ok());

        let stream = result.unwrap();
        assert_eq!(stream.events().len(), 3);
    }
}
