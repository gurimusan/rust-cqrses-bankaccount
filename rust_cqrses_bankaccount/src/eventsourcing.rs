use std::fmt;
use failure::{Fail, Context, Backtrace};
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct EventStream<Event> {
    events: Vec<Event>,
    version: u64,
}

impl<Event> EventStream<Event> {
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

#[derive(Debug, Clone)]
pub struct Snapshot<Data> {
    stream_id: String,
    stream_version: u64,
    snapshot: Data,
    created_at: DateTime<Local>,
}

impl<Data> Snapshot<Data> {
    pub fn new(stream_id: String, stream_version: u64, snapshot: Data, created_at: DateTime<Local>) -> Self {
        Self {
            stream_id,
            stream_version,
            snapshot,
            created_at,
        }
    }

    pub fn stream_id(&self) -> &str {
        &self.stream_id
    }

    pub fn stream_version(&self) -> u64 {
        self.stream_version
    }

    pub fn snapshot(&self) -> &Data {
        &self.snapshot
    }

    pub fn created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }
}

#[derive(Debug)]
pub struct EventStoreError {
    inner: Context<EventStoreErrorKind>,
}

#[derive(Clone, Debug, Fail)]
pub enum EventStoreErrorKind {
    #[fail(display = "No events to append")]
    NoEventsError,

    #[fail(display = "Duplicate entry error: {:?}", _0)]
    DuplicateEntryError(String),

    #[fail(display = "Append event stream error: {:?}", _0)]
    AppendEventStreamError(String),

    #[fail(display = "There is no such event stream: {}:{}", _0, _1)]
    NoEventStreamError(String, u64),

    #[fail(display = "Query error: {:?}", _0)]
    QueryError(String),
}

impl Fail for EventStoreError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for EventStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl EventStoreError {
    pub fn kind(&self) -> &EventStoreErrorKind {
        &self.inner.get_context()
    }
}

impl From<EventStoreErrorKind> for EventStoreError {
    fn from(kind: EventStoreErrorKind) -> EventStoreError {
        EventStoreError { inner: Context::new(kind) }
    }
}

impl From<Context<EventStoreErrorKind>> for EventStoreError {
    fn from(inner: Context<EventStoreErrorKind>) -> EventStoreError {
        EventStoreError { inner: inner }
    }
}

pub trait EventStore: Send + Sync {
    type Event;
    type EventStream;
    type SnapshotData;

    fn append_event_stream(&self, stream_id: String, stream_version: u64, events: Vec<Self::Event>)
        -> Result<(), EventStoreError>;

    fn event_stream_since(&self, stream_id: String, stream_version: u64)
        -> Result<Self::EventStream, EventStoreError>;

    fn record_snapshot(&self, snapshot: Snapshot<Self::SnapshotData>)
        -> Result<(), EventStoreError>;

    fn read_snapshot(&self, stream_id: String)
        -> Result<Option<Snapshot<Self::SnapshotData>>, EventStoreError>;
}

pub trait EventPublisher {
    type Event;

    fn publish(&self, event: Self::Event);
}
