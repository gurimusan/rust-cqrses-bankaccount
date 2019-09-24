use failure::Fail;
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct EventStreamId {
    stream_name: String,
    stream_version: u64,
}

impl EventStreamId {
    pub fn new(stream_name: String, stream_version: u64) -> Self {
        Self {
            stream_name: stream_name,
            stream_version: stream_version,
        }
    }

    pub fn stream_name(&self) -> &str {
        &self.stream_name
    }

    pub fn stream_version(&self) -> u64 {
        self.stream_version
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    event_type: String,
    event_body: String,
    event_occurred_at: DateTime<Local>,
}

impl Event {
    pub fn new(event_type: String, event_body: String, event_occurred_at: DateTime<Local>) -> Self {
        Self {
            event_type: event_type,
            event_body: event_body,
            event_occurred_at: event_occurred_at,
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
}

pub struct EventStream {
    events: Vec<Event>,
    version: u64,
}

impl EventStream {
    pub fn new(events: Vec<Event>, version: u64) -> Self {
        Self {
            events: events,
            version: version,
        }
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn version(&self) -> u64 {
        self.version
    }
}

#[derive(Debug, Fail)]
pub enum EventStoreError {
    #[fail(display = "No events to append")]
    NoEventsError,

    #[fail(display = "Duplicate entry error: {:?}", _0)]
    DuplicateEntryError(String),

    #[fail(display = "Save error: {:?}", _0)]
    SaveError(String),

    #[fail(display = "There is no such event stream: {}:{}", _0, _1)]
    NoEventStreamError(String, u64),

    #[fail(display = "Query error: {:?}", _0)]
    QueryError(String),
}

pub trait EventStore {
    fn save(&self, id: EventStreamId, events: Vec<Event>) -> Result<(), EventStoreError>;

    fn event_stream_since(&self, id: &EventStreamId) -> Result<EventStream, EventStoreError>;
}
